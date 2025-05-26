use aws_config::{self, ConfigLoader}; // Standard way to bring aws_config into scope
use aws_sdk_s3::{Client, Error};
use axum::{Json, Router, extract::Query, http::Method, routing::get};
use log::error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
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
async fn list_s3_keys() -> Result<Json<Vec<KeyInfo>>, String> {
    let loader = ConfigLoader::default();

    let config = loader.load().await;

    let s3 = Client::new(&config);

    let mut response = s3
        .list_objects_v2()
        .bucket("dz-bucket-1234")
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(out) => match out.contents {
                Some(objects) => {
                    for obj in objects {
                        if let Some(name) = obj.key {
                            println!("key {}", name);
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

    let all_keys: Vec<KeyInfo> = Vec::new();

    Ok(Json(all_keys))
}

#[tokio::main]
async fn main() {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Build the application with the new route
    let app = Router::new()
        .route("/api/keys", get(list_s3_keys)) // <-- NEW ROUTE
        .layer(cors);

    // Run it on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend server listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
