use aws_sdk_s3::primitives::DateTime;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct KeyInfo {
    pub key: String,
    pub last_modified: String,
    pub size: i64,
    #[serde(skip_serializing)]
    pub _last_modified_dt: DateTime,
}
