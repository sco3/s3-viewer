use gloo_console; // Import gloo_console for error logging
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*; // Import Request from gloo_net

use crate::components::keylist::KeyList;
use crate::models::keyinfo::KeyInfo;

#[function_component(App)]
pub fn app() -> Html {
    // Initialize keys with an empty Vec.
    // use_state accepts a closure `|| T` or a direct value `T`.
    let keys = use_state(|| Vec::<KeyInfo>::new()); // Changed to a closure returning a new Vec

    {
        // Clone the state handle for use inside the effect's closure
        let keys_setter = keys.clone();
        // The correct order for use_effect_with is (dependencies, closure_returning_cleanup)
        use_effect_with(
            (), // Dependencies: `()` means run once after first render
            move |_| {
                // The closure that defines the effect
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
                // The cleanup function. For a simple fetch-once effect, it's empty.
                || ()
            },
        );
    }

    html! {
        <div>
            <h1>{ "S3 Viewer" }</h1>
            // Pass a clone of the dereferenced keys UseStateHandle content
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
