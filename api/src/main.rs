use axum::extract::Path;
use axum::http::{header, HeaderMap};
use axum::{extract::Extension, http::StatusCode, routing::any, AddExtensionLayer, Router, Server};
use clap::Parser;
use lightit::{Lamp, State};
use options::Options;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{error::Error, net::SocketAddr};
use tokio::sync::Mutex;

mod options;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();
    let app = Router::new()
        .route("/:state", any(switch))
        .layer(AddExtensionLayer::new(Lamp(options.endpoint)))
        .layer(AddExtensionLayer::new(options.authorization))
        .layer(AddExtensionLayer::new(options.cooldown))
        .layer(AddExtensionLayer::new(Arc::new(Mutex::new(
            Instant::now() - Duration::from_secs(60 * options.cooldown.unwrap_or(0)),
        ))));
    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn switch(
    Path(state): Path<String>,
    headers: HeaderMap,
    Extension(lamp): Extension<Lamp>,
    Extension(authorization): Extension<Option<String>>,
    Extension(cooldown): Extension<Option<u64>>,
    Extension(last_change): Extension<Arc<Mutex<Instant>>>,
) -> Result<&'static str, (StatusCode, String)> {
    let state = match state.parse::<State>() {
        Ok(state) => state,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    // Using authorization header.
    if let (Some(header), Some(authorization)) = (
        headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok()),
        authorization,
    ) {
        return if header == authorization {
            set_state(lamp, state).await
        } else {
            Err((
                StatusCode::FORBIDDEN,
                "invalid authorization header".to_owned(),
            ))
        };
    }

    // Using public cooldown controlled call.
    if let Some(cooldown) = cooldown {
        let mut last_change = last_change.lock().await;
        return if last_change.elapsed().as_secs() >= 60 * cooldown {
            *last_change = Instant::now();
            set_state(lamp, state).await
        } else {
            Err((
                StatusCode::TOO_MANY_REQUESTS,
                "please wait between actions".to_owned(),
            ))
        };
    }

    Err((
        StatusCode::BAD_REQUEST,
        "invalid or forbidden request".to_owned(),
    ))
}

async fn set_state(lamp: Lamp, state: State) -> Result<&'static str, (StatusCode, String)> {
    lamp.set_state(state)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok("ok")
}
