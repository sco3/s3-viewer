use std::time::Instant;

use axum::{extract::State, Json};
use log::error;

use crate::appstate::AppState;
use crate::keyinfo::KeyInfo;
use crate::pushentry;

fn last_modified_desc(a: &KeyInfo, b: &KeyInfo) -> std::cmp::Ordering {
    b.last_modified.cmp(&a.last_modified) // descending
}

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

    all_keys.sort_by(last_modified_desc);

    println!("Took: {} ms", start.elapsed().as_millis());
    Ok(Json(all_keys))
}
