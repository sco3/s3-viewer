use gloo_console; // Import gloo_console for error logging
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::keylist::KeyList;
use crate::models::keyinfo::KeyInfo;

#[function_component(App)]
pub fn app() -> Html {
    let keys = use_state(Vec::<KeyInfo>::new); // Changed to a closure returning a new Vec

    let keys_setter = keys.clone();
    use_effect_with((), move |_| {
        spawn_local(async move {
            match fetch_keys().await {
                Ok(fetched_keys) => {
                    keys_setter.set(fetched_keys); // Use the setter
                }
                Err(err) => {
                    gloo_console::error!(format!("Failed to fetch keys: {:?}", err));
                }
            }
        });
        || ()
    });

    html! {
        <div>
            <h1>{ "S3 Viewer" }</h1>
            <KeyList keys={(*keys).clone()} />
        </div>
    }
}

// Helper async function to fetch keys
async fn fetch_keys() -> Result<Vec<KeyInfo>, gloo_net::Error> {
    let resp = Request::get("/api/keys") // Use Request directly as it's imported
        .send()
        .await?;
    resp.json::<Vec<KeyInfo>>().await
}
