use clap::Parser;
use lightit::Lamp;
use options::Options;
use scheduler::Scheduler;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

mod options;
mod scheduler;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let options = Options::parse();
    let lamp = Lamp(options.endpoint);

    let mut scheduler = Scheduler {
        latitude: options.latitude,
        longitude: options.longitude,
        offset: options.offset,
        weekdays: options.weekdays.0,
        fixed_off: options.fixed_off,
    };
    loop {
        if let Some((duration, state)) = scheduler.next() {
            sleep(duration).await;
            if let Err(err) = lamp.set_state(state).await {
                eprintln!("{}", err.to_string());
            }
        } else {
            eprintln!("cannot determine next state change");
            sleep(Duration::from_secs(5 * 60)).await;
        }
    }
}
