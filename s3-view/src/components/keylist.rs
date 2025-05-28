use crate::components::fileviewer::FileViewer;
use crate::models::keyinfo::KeyInfo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyListProps {
    pub keys: Vec<KeyInfo>,
}

#[function_component(KeyList)]
pub fn key_list(props: &KeyListProps) -> Html {
    let selected_key = use_state(|| None::<String>);

    let on_key_click = {
        let selected_key = selected_key.clone();
        Callback::from(move |key: String| {
            selected_key.set(Some(key));
        })
    };

    html! {
        <>
        if let Some(key) = &*selected_key {
            <div class="mt-4">
                //<FileViewer file_key={key.clone()} />
                <FileViewer
            file_key={key.clone()}
            on_close={Callback::from(move |_| selected_key.set(None))}
        />
            </div>
        } else {
            <div class="overflow-x-auto">
                <table class="min-w-full table-auto border-collapse">
                    <thead class="bg-gray-100">
                        <tr>
                            <th class="border border-gray-300 px-6 py-3 text-left text-xs font-medium text-gray-700 uppercase tracking-wider">
                                { "Key" }
                            </th>
                            <th class="border border-gray-300 px-6 py-3 text-left text-xs font-medium text-gray-700 uppercase tracking-wider">
                                { "Last Modified" }
                            </th>
                            <th class="border border-gray-300 px-6 py-3 text-right text-xs font-medium text-gray-700 uppercase tracking-wider">
                                { "Size (bytes)" }
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        { for props.keys.iter().map(|key| {
                            let key_name = key.key.clone();
                            html! {
                                <tr class="hover:bg-gray-50 even:bg-white odd:bg-gray-50">
                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                        <a href="#" onclick={
                                            let key_name = key_name.clone();
                                            let on_key_click = on_key_click.clone();
                                            Callback::from(move |e: MouseEvent| {
                                                e.prevent_default(); // Prevent default link behavior
                                                on_key_click.emit(key_name.clone());
                                            })
                                        } class="text-blue-600 hover:text-blue-800">
                                            { &key.key }
                                        </a>
                                    </td>
                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                                        { &key.last_modified }
                                    </td>
                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600 text-right">
                                        { &key.size }
                                    </td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        }
        </>
    }
}
