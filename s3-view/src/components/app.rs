use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::fetchkeys::fetch_keys;
use crate::components::keylist::KeyList;
use crate::models::keyinfo::KeyInfo;

#[function_component(App)]
pub fn app() -> Html {
    let keys: UseStateHandle<Vec<KeyInfo>> = use_state(Vec::new);

    {
        let keys = keys.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
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
            <h1 class="text-2xl font-bold mb-4">{ "S3 Viewer" }</h1>
            <KeyList keys={(*keys).clone()} />
        </div>
    }
}
