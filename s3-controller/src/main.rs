use appstate::AppState;
use aws_config::{self, ConfigLoader}; // Standard way to bring aws_config into scope
use aws_sdk_s3::{primitives::DateTime, Client};
use axum::{extract::State, http::Method, routing::get, Router};
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
    time::Instant,
};
use tower_http::cors::{Any, CorsLayer};

mod appstate;
mod keyinfo;
mod pushentry;

// (Optional) Query parameters - we'll activate these later
#[derive(Deserialize, Debug)]
struct ListKeysParams {
    prefix: Option<String>,
    page_token: Option<String>,
    limit: Option<i32>,
}

const BUCKET_NAME: &str = "dz-bucket-1234"; // Your bucket name

mod listkeys;

#[tokio::main]
async fn main() {
    let loader = ConfigLoader::default();
    println!("loader ready");
    let config = loader.load().await;
    println!("config ready");
    let s3_result = catch_unwind(AssertUnwindSafe(|| Client::new(&config)));
    match s3_result {
        Ok(s3) => {
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
            .unwrap();
        }
        Err(e) => {
            error!("Error get s3 client :{:?}", e);
        }
    }
}
