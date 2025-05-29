use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub struct Config {
    pub bucket: String,
}
