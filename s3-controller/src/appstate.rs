use std::sync::Arc;
use aws_sdk_s3::Client;

#[derive(Clone)]
pub struct AppState {
    pub s3: Arc<Client>,
    pub bucket: String,
}