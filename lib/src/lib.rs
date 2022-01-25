use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use rand::seq::IteratorRandom;
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum State {
    Off,
    On,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            _ => Err(Error::InvalidState),
        }
    }
}

impl From<State> for u8 {
    fn from(state: State) -> Self {
        match state {
            State::Off => 0,
            State::On => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lamp(pub String);

impl Lamp {
    pub async fn set_state(&self, state: State) -> Result<(), Error> {
        let message_id = {
            let mut rng = rand::thread_rng();
            (0..32)
                .map(|_| {
                    ('0'..='9')
                        .chain('a'..='f')
                        .choose(&mut rng)
                        .ok_or(Error::MessageIdGenerationFailure)
                })
                .collect::<Result<String, _>>()?
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| Error::TimestampGenerationFailure { source: err })?
            .as_secs();
        let payload = json!({
            "header":{
                "messageId": message_id,
                "namespace": "Appliance.Control.ToggleX",
                "method": "SET",
                "payloadVersion": 1,
                "from": "/appliance/1907186973974025184948e1e9014e52/subscribe",
                "timestamp": timestamp,
                "sign": "a76c960ab1cf55497c257b7f8434ed6a"
            },
            "payload":{
                "togglex": {
                    "channel": 0,
                    "onoff": u8::from(state)
                }
            }
        })
        .to_string();
        let response = reqwest::Client::new()
            .post(&format!("{}/config", self.0))
            .body(payload)
            .send()
            .await
            .map_err(|err| Error::HttpCallFailure { source: err })?;
        if response.status() != StatusCode::OK {
            return Err(Error::InvalidStatusCode {
                code: response.status(),
            });
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid state")]
    InvalidState,
    #[error("cannot generate message id")]
    MessageIdGenerationFailure,
    #[error("cannot generate unix timestamp")]
    TimestampGenerationFailure { source: SystemTimeError },
    #[error("invalid api http status code")]
    HttpCallFailure { source: reqwest::Error },
    #[error("invalid status code {code}")]
    InvalidStatusCode { code: StatusCode },
}
