use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct KeyInfo {
    pub key: String,
    pub last_modified: String,
    pub size: i64,
}
