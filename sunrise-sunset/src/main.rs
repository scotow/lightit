use chrono::{DateTime, Datelike, Local, Timelike};
use clap::Parser;
use std::time::Duration;
use std::{env::args, error::Error};
use tokio::time::sleep;

use lightit::{Lamp, State};
use options::Options;

mod options;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let options = Options::parse();
    let lamp = Lamp(options.endpoint);
    let mut current_state = current(options.latitude, options.longitude, options.fixed_off);
    loop {
        sleep(Duration::from_secs(55)).await;
        let new_state = current(options.latitude, options.longitude, options.fixed_off);
        if new_state != current_state {
            if let Err(err) = lamp.set_state(new_state).await {
                eprintln!("{}", err.to_string());
            }
            current_state = new_state;
        }
    }
}

fn current(latitude: f64, longitude: f64, fixed_off: Option<u32>) -> State {
    let now: DateTime<Local> = Local::now();
    let (sunrise, sunset) =
        sunrise::sunrise_sunset(latitude, longitude, now.year(), now.month(), now.day());
    if let Some(fixed_off) = fixed_off {
        if ((fixed_off as i64)..=sunrise).contains(&(now.hour() as i64)) {
            return State::Off;
        }
    }
    if (sunrise..=sunset).contains(&now.timestamp()) {
        State::Off
    } else {
        State::On
    }
}
