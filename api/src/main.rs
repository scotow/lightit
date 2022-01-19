use std::{convert::Infallible, env::args, error::Error};

use lightit::{Lamp, State};
use warp::Filter;

// use axum::{
//     extract::Extension,
//     http::StatusCode,
//     routing::{any, get},
//     AddExtensionLayer, Router,
// };
// use lightit::{Lamp, State};
// use std::{env::args, error::Error, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut args = args();
    args.next();

    let lamp = Lamp(args.next().ok_or("missing lamp endpoint")?);
    let routes = warp::get()
        .and(warp::path::end())
        .map(move || lamp.clone())
        .and_then(turn_off);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

async fn turn_off(lamp: Lamp) -> Result<impl warp::Reply, Infallible> {
    lamp.set_state(State::Off).await;
    Ok("test".to_owned())
}

// #[tokio::main]
// async fn main() {
//     let http_client = reqwest::Client::new();

//     // build our application with a route
//     let app = Router::new()
//         .route("/", get(handler))
//         .layer(AddExtensionLayer::new(http_client));

//     // run it
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn handler(Extension(http_client): Extension<reqwest::Client>) -> Result<String, StatusCode> {
//     let response = http_client
//         .get("http://example.com")
//         .send()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let body = response
//         .text()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(body)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + 'static>> {
//     let mut args = args();
//     args.next();
//     let lamp = Lamp(args.next().ok_or("missing lamp endpoint")?);

//     let app = Router::new()
//         .route("/off", get(off2))
//         .layer(AddExtensionLayer::new(lamp));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
//     Ok(())
// }

// async fn off2(Extension(lamp): Extension<Lamp>) {
//     // let lamp = lamp.clone();
//     // dbg!(lamp);
//     lamp.set_state(State::Off).await.unwrap();
//     // let it2 = it.as_ref().it;
//     // dbg!(it2);
//     // Ok("Ok".to_owned())
// }

// // async fn off(Extension(lamp): Extension<Lamp>) -> Result<String, String> {
// //     lamp.set_state(State::Off)
// //         .await
// //         .map_err(|err| err.to_string())?;
// //     Ok("Ok".to_owned())
// // }

// async fn on(Extension(lamp): Extension<Lamp>) -> Result<String, String> {
//     lamp.set_state(State::On)
//         .await
//         .map_err(|err| err.to_string())?;
//     Ok("Ok".to_owned())
// }
