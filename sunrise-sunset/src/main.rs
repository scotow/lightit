use clap::Parser;
use lightit::Lamp;
use options::Options;
use scheduler::Scheduler;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

mod options;
mod scheduler;

macro_rules! verbose {
    ($opt: expr, $($arg : tt) *) => {
        if $opt.verbose {
            eprintln!($($arg)*);
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let options = Options::parse();
    let lamp = Lamp(options.endpoint);

    let scheduler = Scheduler {
        latitude: options.latitude,
        longitude: options.longitude,
        offset: options.offset,
        weekdays: options.weekdays.0,
        fixed_off: options.fixed_off,
    };
    loop {
        if let Some((duration, state)) = scheduler.next() {
            verbose!(
                options,
                "waiting {:?} before changing state to {:?}",
                duration,
                state
            );
            sleep(duration).await;
            if let Err(err) = lamp.set_state(state).await {
                verbose!(options, "failed to change lamp state: {}", err.to_string());
            } else {
                verbose!(options, "state changed to {:?}", state);
            }
        } else {
            verbose!(options, "cannot determine next state change");
            sleep(Duration::from_secs(60 * 5)).await;
        }
    }
}
