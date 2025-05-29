use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ListKeysParams {
    pub(crate) _prefix: Option<String>,
    pub(crate) _page_token: Option<String>,
    pub(crate) _limit: Option<i32>,
}
