use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

// Define a struct that matches the JSON structure from our backend
#[derive(Clone, PartialEq, Deserialize)]
struct BucketInfo {
    name: String,
    created_at: String,
}

#[function_component(App)]
fn app() -> Html {
    // Create a state handle to store our list of buckets.
    // It starts as an empty vector.
    let buckets = use_state(|| Vec::<BucketInfo>::new());
    // Create a state handle for potential errors.
    let error = use_state(|| None::<String>);

    // `use_effect_with_deps` runs its code once when the component mounts
    // (because the dependency list `()` is empty and never changes).
    {
        // We need to clone `buckets` and `error` so we can move them into the async block.
        let buckets = buckets.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            // This `spawn_local` runs our async code.
            spawn_local(async move {
                let fetched_buckets: Result<Vec<BucketInfo>, reqwasm::Error> = Request::get("http://localhost:3000/api/buckets")
                    .send()
                    .await
                    .map_err(|e| {
                        error.set(Some(format!("Failed to send request: {}", e)));
                        e // Keep the original error type for the next map_err
                    })
                    .and_then(|resp| {
                        if !resp.ok() {
                            Err(reqwasm::Error::new(format!("Request failed with status: {}", resp.status())))
                        } else {
                            Ok(resp)
                        }
                    })
                    .and_then(|resp| {
                        Box::pin(async move { resp.json::<Vec<BucketInfo>>().await })
                    })
                    .await; // Await the result of the JSON parsing


                match fetched_buckets {
                    Ok(b) => {
                        buckets.set(b); // Update the state with fetched buckets
                        error.set(None); // Clear any previous error
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch or parse buckets: {}", e)));
                    }
                }
            });
            || () // Cleanup function (not needed here)
        }, ()); // Empty dependency list means run once.
    }

    // Render the component
    html! {
        <div>
            <h1>{ "Rust Yew S3 Viewer" }</h1>
            
            { 
                if let Some(err_msg) = &*error {
                    html! { <p style="color: red;">{ format!("Error: {}", err_msg) }</p> }
                } else {
                    html! {}
                }
            }

            <h2>{ "Available S3 Buckets:" }</h2>
            { 
                if buckets.is_empty() && error.is_none() {
                    html! { <p>{ "Loading buckets..." }</p> }
                } else {
                    html! {
                        <ul>
                            { 
                                buckets.iter().map(|bucket| html! {
                                    <li>{ format!("{} (Created: {})", &bucket.name, &bucket.created_at) }</li>
                                }).collect::<Html>() 
                            }
                        </ul>
                    }
                }
            }
        </div>
    }
}

// Keep the wasm_bindgen entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}