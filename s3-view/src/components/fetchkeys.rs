use gloo_console::error;
use gloo_net::http::Request;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::models::keyinfo::KeyInfo;

#[hook]
pub fn use_fetch_keys() -> UseStateHandle<Option<Vec<KeyInfo>>> {
    let keys = use_state_eq(|| None);

    {
        let keys = keys.clone(); // Clone the state handle for use inside the closure
        use_effect_with(
            (), // Dependencies: `()` means run once after the first render
            move |_| {
                let keys = keys.clone(); // Clone again for the async block
                spawn_local(async move {
                    match Request::get("/api/keys").send().await {
                        Ok(response) => match response.json::<Vec<KeyInfo>>().await {
                            Ok(data) => keys.set(Some(data)),
                            Err(_) => {
                                error!("Failed to parse JSON response");
                            }
                        },
                        Err(_) => {
                            error!("Failed to fetch keys from API");
                        }
                    }
                });

                || () // Cleanup function (no-op in this case)
            },
        );
    }

    keys
}