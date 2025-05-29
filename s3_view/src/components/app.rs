use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::fetchcfg::fetch_cfg;
use crate::components::fetchkeys::fetch_keys;
use crate::components::keylist::KeyList;

use crate::models::keyinfo::KeyInfo;

#[function_component(App)]
pub fn app() -> Html {
    let keys: UseStateHandle<Vec<KeyInfo>> = use_state(Vec::new);
    let bucket = use_state(|| String::new());

    {
        let keys = keys.clone();
        let bucket = bucket.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_cfg().await {
                    Ok(fetched_config) => {
                        bucket.set(fetched_config.bucket);
                    }
                    Err(err) => {
                        log!(format!("Failed to fetch cfg: {:?}", err));
                    }
                }
                match fetch_keys().await {
                    Ok(fetched_keys) => {
                        keys.set(fetched_keys);
                    }
                    Err(err) => {
                        log!(format!("Failed to fetch keys: {:?}", err));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="p-4">
        <h1 class="text-2xl font-bold mb-4">{ "S3 Viewer: " } {(*bucket).clone()} </h1>
            <KeyList keys={(*keys).clone()} />
        </div>
    }
}
