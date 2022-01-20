use chrono::{DateTime, Datelike, Local};
use std::time::Duration;
use std::{env::args, error::Error};
use tokio::time::sleep;

use lightit::{Lamp, State};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut args = args();
    args.next();
    let lamp = Lamp(args.next().ok_or("missing lamp endpoint")?);
    let latitude = args
        .next()
        .map(|n| n.parse().ok())
        .flatten()
        .ok_or("missing latitude")?;
    let longitude = args
        .next()
        .map(|n| n.parse().ok())
        .flatten()
        .ok_or("missing longitude")?;

    let mut current_state = current(latitude, longitude);
    loop {
        sleep(Duration::from_secs(120)).await;
        let new_state = current(latitude, longitude);
        if new_state != current_state {
            if let Err(err) = lamp.set_state(new_state).await {
                eprintln!("{}", err.to_string());
            }
            current_state = new_state;
        }
    }
}

fn current(latitude: f64, longitude: f64) -> State {
    let now: DateTime<Local> = Local::now();
    let (sunrise, sunset) =
        sunrise::sunrise_sunset(latitude, longitude, now.year(), now.month(), now.day());
    if (sunrise..=sunset).contains(&now.timestamp()) {
        State::Off
    } else {
        State::On
    }
}
