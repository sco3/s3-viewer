use aws_sdk_s3::Client;
use std::sync::Arc;

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]

pub struct AppState {
    #[serde(skip_serializing)]
    pub s3: Arc<Client>,
    pub bucket: String,
}
