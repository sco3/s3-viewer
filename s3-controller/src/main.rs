use appstate::AppState;
use aws_config::{self, ConfigLoader};
use aws_sdk_s3::Client;
use axum::{http::Method, routing::get, Router};
use log::error;

use std::{
    net::SocketAddr,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};
use tower_http::cors::{Any, CorsLayer};

mod appstate;
mod keyinfo;
mod pushentry;

mod listkeyparam;

const BUCKET_NAME: &str = "dz-bucket-1234";

mod listkeys;

#[tokio::main]
async fn main() {
    let config = ConfigLoader::default().load().await;
    let s3 = match catch_unwind(
        AssertUnwindSafe(|| Client::new(&config)), //
    ) {
        Ok(cli) => cli,
        Err(e) => {
            error!("Error get s3 client {:?}", e);
            return;
        }
    };

    let state = AppState {
        s3: Arc::new(s3),
        bucket: BUCKET_NAME.to_string(),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Build the application with the new route
    let app = Router::new()
        .route("/api/keys", get(listkeys::list_s3_keys)) //
        .with_state(state.clone())
        .layer(cors);

    // Run it on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr) //
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap_or_else(|e| eprintln!("Serve error: {:?}", e));
}
