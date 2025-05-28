use std::time::Instant;

use axum::extract::{Path, State};
use axum::response::Response;

use crate::appstate::AppState;

pub(crate) async fn view_s3_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Response {
    let start = Instant::now();
    println!("Fetching key: {}", key);

    match state
        .s3
        .get_object()
        .bucket(&state.bucket)
        .key(&key)
        .send()
        .await
    {
        Ok(resp) => {
            println!("Key fetched successfully: {}", key);

            match resp.body.collect().await {
                Ok(body) => {
                    println!("Get object took: {} ms", start.elapsed().as_millis());
                    Response::builder()
                        .status(200)
                        .body(body.into_bytes().into())
                        .unwrap()
                }
                Err(e) => {
                    let msg = format!("Error collecting body: {}", e);
                    Response::builder().status(500).body(msg.into()).unwrap()
                }
            }
        }
        Err(e) => {
            let msg = format!("Error fetching key {}: {}", key, e);
            eprintln!("{}", msg);
            Response::builder().status(500).body(msg.into()).unwrap()
        }
    }
}
