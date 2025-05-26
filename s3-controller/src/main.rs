use aws_config::{self, ConfigLoader}; // Standard way to bring aws_config into scope
use aws_sdk_s3::{primitives::DateTime, Client};
use axum::{extract::State, http::Method, routing::get, Json, Router};
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
    time::Instant,
};
use tower_http::cors::{Any, CorsLayer};

// Define a struct for Key information
#[derive(Serialize, Clone, Debug)]
struct KeyInfo {
    key: String,
    last_modified: String,
    size: i64,
    // Store original DateTime for sorting
    #[serde(skip_serializing)] // Don't send this in JSON
    last_modified_dt: aws_sdk_s3::primitives::DateTime,
}

// (Optional) Query parameters - we'll activate these later
#[derive(Deserialize, Debug)]
struct ListKeysParams {
    prefix: Option<String>,
    page_token: Option<String>,
    limit: Option<i32>,
}

const BUCKET_NAME: &str = "dz-bucket-1234"; // Your bucket name

// Handler function to list S3 Keys
async fn list_s3_keys(State(s3): State<Arc<Client>>) -> Result<Json<Vec<KeyInfo>>, String> {
    let start = Instant::now();
    let mut all_keys: Vec<KeyInfo> = Vec::new();
    let mut response = s3
        .list_objects_v2()
        .bucket(BUCKET_NAME)
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(out) => match out.contents {
                Some(objects) => {
                    for obj in objects {
                        if let Some(name) = obj.key {
                            let info = KeyInfo {
                                key: name,
                                last_modified: "".into(),
                                size: 0,
                                last_modified_dt: DateTime::from_millis(0),
                            };
                            all_keys.push(info);
                        }
                    }
                }
                None => {
                    error!("No objects");
                }
            },
            Err(e) => {
                error!("Fail: {:?}", e)
            }
        }
    }
    let out = Json(all_keys);
    println!("Took: {}", start.elapsed().as_millis());
    Ok(out)
}

#[tokio::main]
async fn main() {
    let loader = ConfigLoader::default();
    println!("loader ready");
    let config = loader.load().await;
    println!("config ready");
    let s3_result = catch_unwind(AssertUnwindSafe(|| Client::new(&config)));
    match s3_result {
        Ok(s3) => {
            let shared_s3 = Arc::new(s3);
            // CORS layer
            let cors = CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any);

            // Build the application with the new route
            let app = Router::new()
                .route("/api/keys", get(list_s3_keys)) //
                .with_state(shared_s3)
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
