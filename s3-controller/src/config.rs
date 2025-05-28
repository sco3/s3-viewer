use axum::{extract::State, Json};

use crate::appstate::AppState;

pub(crate) async fn get_cfg(State(state): State<AppState>, //
) -> Result<Json<AppState>, String> {
    println!("bucket: {}", state.bucket);
    Ok(Json(state))
}
