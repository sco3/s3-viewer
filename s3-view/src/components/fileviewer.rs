use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_json_viewer::JsonViewer;

#[derive(Properties, PartialEq)]
pub struct FileViewerProps {
    pub file_key: String,
    pub on_close: Callback<()>,
}

#[function_component(FileViewer)]
pub fn file_viewer(props: &FileViewerProps) -> Html {
    let file_content = use_state(|| None::<String>);
    let json_data = use_state(|| None::<serde_json::Value>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let file_key = props.file_key.clone();
        let file_content = file_content.clone();
        let json_data = json_data.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let url = format!("/api/view/{}", file_key);

                match Request::get(&url).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.text().await {
                                Ok(text) => match serde_json::from_str::<Value>(&text) {
                                    Ok(json_value) => {
                                        json_data.set(Some(json_value));
                                        file_content.set(None);
                                    }
                                    Err(_) => {
                                        file_content.set(Some(text));
                                        json_data.set(None);
                                    }
                                },
                                Err(e) => error.set(Some(format!("Failed to parse response: {e}"))),
                            }
                        } else {
                            error.set(Some(format!("HTTP error: {}", response.status())));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Network error: {e}")));
                    }
                }

                loading.set(false);
            });

            || ()
        });
    }

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div>
            <div class="flex justify-between items-center mb-2">
                <h2 class="text-lg font-bold">{ &props.file_key }</h2>
                <button
                    class="text-xl font-bold px-2 py-1 hover:text-red-600"
                    onclick={on_close_click}
                    aria-label="Close viewer"
                >
                    { "×" }
                </button>
            </div>

            {
                if *loading {
                    html! { <p>{ "Loading..." }</p> }
                } else if let Some(err) = &*error {
                    html! { <p class="text-red-600">{ err }</p> }
                } else if let Some(json) = &*json_data {
                    // Render JSON nicely using JsonViewer
                    JsonViewer::new(json.clone()).render()
                } else if let Some(text) = &*file_content {

                    html! { <pre class="bg-gray-100 p-4 rounded whitespace-pre-wrap break-words">{ text }</pre> }
                } else {
                    html! { <p>{ "No content available." }</p> }
                }
            }
        </div>
    }
}
