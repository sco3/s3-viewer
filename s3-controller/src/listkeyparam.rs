use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ListKeysParams {
    pub(crate) prefix: Option<String>,
    pub(crate) page_token: Option<String>,
    pub(crate) limit: Option<i32>,
}
