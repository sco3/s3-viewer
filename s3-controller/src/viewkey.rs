use std::time::Instant;

use axum::extract::{Path, State};
use axum::response::Response;

use crate::appstate::AppState;

pub(crate) async fn view_s3_key(
    State(_state): State<AppState>,
    Path(key): Path<String>,
) -> Response {
    let start = Instant::now();
    println!("Fetching key: {}", key);

    println!("Took: {} ms", start.elapsed().as_millis());

    return Response::builder()
        .status(500)
        .body("Not implemented".into())
        .unwrap();
}
