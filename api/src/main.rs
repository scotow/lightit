use axum::response::IntoResponse;
use axum::{extract::Extension, http::StatusCode, routing::any, AddExtensionLayer, Router, Server};
use lightit::{Lamp, State};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{env::args, error::Error, net::SocketAddr};
use tokio::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    args.next();
    let lamp = Lamp(args.next().ok_or("missing lamp endpoint")?);

    let app = Router::new()
        .route("/off", any(off))
        .route("/on", any(on))
        .layer(AddExtensionLayer::new(lamp))
        .layer(AddExtensionLayer::new(Arc::new(Mutex::new(
            Instant::now() - Duration::from_secs(60 * 60),
        ))));
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn off(
    Extension(lamp): Extension<Lamp>,
    Extension(last_change): Extension<Arc<Mutex<Instant>>>,
) -> impl IntoResponse {
    state(&lamp, State::Off, last_change).await
}

async fn on(
    Extension(lamp): Extension<Lamp>,
    Extension(last_change): Extension<Arc<Mutex<Instant>>>,
) -> impl IntoResponse {
    state(&lamp, State::On, last_change).await
}

async fn state(
    lamp: &Lamp,
    state: State,
    last_change: Arc<Mutex<Instant>>,
) -> Result<&'static str, (StatusCode, String)> {
    let mut last_change = last_change.lock().await;
    if last_change.elapsed().as_secs() < 300 {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "please wait between actions".to_owned(),
        ));
    }
    lamp.set_state(state)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    *last_change = Instant::now();
    Ok("ok")
}
