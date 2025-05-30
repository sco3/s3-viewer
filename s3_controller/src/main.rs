mod appstate;
mod args;
mod config;
mod keyinfo;
mod listkeyparam;
mod listkeys;
mod pushentry;
mod viewkey;

use appstate::AppState;
use aws_config::{self, meta::region::RegionProviderChain, ConfigLoader, Region};
use aws_sdk_s3::Client;
use axum::{http::Method, routing::get, Router};
use config::get_cfg;

use log::error;
use tower_http::services::ServeDir;

use axum_server::tls_rustls::RustlsConfig;

use std::{
    net::SocketAddr,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};
use tower_http::cors::{Any, CorsLayer};

use include_dir::{include_dir, Dir};

use crate::args::Args;
use clap::Parser;
use tempfile::TempDir;

const STATIC: Dir = include_dir!("static");

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let region_provider = RegionProviderChain::first_try(
        Some(Region::new(args.region.clone())), //
    )
    .or_default_provider();

    let config = ConfigLoader::default().region(region_provider).load().await;

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
        bucket: args.bucket,
    };

    let _cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    STATIC
        .extract(temp_dir.path())
        .expect("Failed to extract static files");

    // Build the application with the new route
    let app = Router::new()
        .route(
            "/api/keys",                 //
            get(listkeys::list_s3_keys), //
        )
        .route(
            "/api/view/{*key}",
            get(viewkey::view_s3_key), //
        )
        .route(
            "/api/cfg",
            get(get_cfg), //
        )
        .with_state(state.clone())
        .fallback_service(
            ServeDir::new(temp_dir.path()) //
                .append_index_html_on_directories(true),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("Backend server listening on {}", addr);

    if args.tls {
        match RustlsConfig::from_pem_file(args.cert_path_tls, args.key_path_tls).await {
            Ok(config) => {
                axum_server::bind_rustls(addr, config)
                    .serve(app.into_make_service())
                    .await
                    .unwrap_or_else(|e| eprintln!("TLS Server error: {:?}", e));
            }
            Err(e) => {
                eprintln!("Cannot load tls files: {}", e)
            }
        }
    } else {
        axum::serve(
            tokio::net::TcpListener::bind(addr) //
                .await
                .unwrap(),
            app,
        )
        .await
        .unwrap_or_else(|e| eprintln!("Serve error: {:?}", e));
    }
}
