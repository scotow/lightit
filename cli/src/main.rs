use std::{env::args, error::Error};

use lightit::Lamp;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut args = args();
    args.next();
    let lamp = args.next().ok_or("missing lamp endpoint")?;
    let state = args
        .next()
        .ok_or("missing target state")?
        .parse()
        .map_err(|_| "invalid lamp state")?;
    let lamp = Lamp(lamp);
    lamp.set_state(state).await?;
    Ok(())
}
