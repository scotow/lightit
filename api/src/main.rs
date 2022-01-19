use axum::response::IntoResponse;
use axum::{extract::Extension, http::StatusCode, routing::any, AddExtensionLayer, Router, Server};
use lightit::{Lamp, State};
use std::{env::args, error::Error, net::SocketAddr};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    args.next();
    let lamp = Lamp(args.next().ok_or("missing lamp endpoint")?);

    let app = Router::new()
        .route("/off", any(off))
        .route("/on", any(on))
        .layer(AddExtensionLayer::new(lamp));
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn off(Extension(lamp): Extension<Lamp>) -> impl IntoResponse {
    state(&lamp, State::Off).await
}

async fn on(Extension(lamp): Extension<Lamp>) -> impl IntoResponse {
    state(&lamp, State::On).await
}

async fn state(lamp: &Lamp, state: State) -> Result<&'static str, (StatusCode, String)> {
    lamp.set_state(state)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok("ok")
}
