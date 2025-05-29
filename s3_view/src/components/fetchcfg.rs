use crate::models::cfg::Config;
use gloo_console::log;
use gloo_net::http::Request;

pub async fn fetch_cfg() -> Result<Config, gloo_net::Error> {
    log!("Fetching keys check");
    let resp = Request::get("/api/cfg").send().await?;
    resp.json::<Config>().await
}
