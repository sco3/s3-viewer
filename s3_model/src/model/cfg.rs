
use serde::Deserialize;
use serde::Serialize;

const VERSION = "0.1.1"

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Cfg {
    #[serde(default = "Cfg::default_version")]
    pub version: String,
    pub bucket: String,
}

impl Cfg {

    fn default_version() -> String {
        VERSION.to_string()
    }

    pub fn new(bucket: String) -> Self {
        Self {
            version: VERSION,
            bucket,
        }
    }
}