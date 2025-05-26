use std::time::Instant;

use axum::{extract::State, Json};
use log::error;

use crate::appstate::AppState;
use crate::keyinfo::KeyInfo;
use crate::pushentry;

pub(crate) async fn list_s3_keys(
    State(state): State<AppState>,
) -> Result<Json<Vec<KeyInfo>>, String> {
    let start = Instant::now();
    let mut all_keys: Vec<KeyInfo> = Vec::new();

    let mut response = state
        .s3
        .list_objects_v2()
        .bucket(&state.bucket)
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(out) => match out.contents {
                Some(objects) => {
                    for obj in objects {
                        pushentry::push_entry(&mut all_keys, obj);
                    }
                }
                None => {
                    error!("No objects found in the bucket");
                }
            },
            Err(e) => {
                error!("Error listing objects: {:?}", e);
            }
        }
    }

    println!("Took: {} ms", start.elapsed().as_millis());
    Ok(Json(all_keys))
}
