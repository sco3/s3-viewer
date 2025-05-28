use gloo_console::log;
use gloo_net::http::Request;
use crate::models::keyinfo::KeyInfo;

pub async fn fetch_keys() -> Result<Vec<KeyInfo>, gloo_net::Error> {
    log!("Fetching keys check");
    let resp = Request::get("/api/keys").send().await?;
    resp.json::<Vec<KeyInfo>>().await
}
