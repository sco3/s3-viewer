// src/components/keylist.rs
use crate::models::keyinfo::KeyInfo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyListProps {
    pub keys: Vec<KeyInfo>,
}

#[function_component(KeyList)]
pub fn key_list(props: &KeyListProps) -> Html {
    html! {
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
                    { for props.keys.iter().map(|key| html! {

                          <tr class="hover:bg-gray-50 even:bg-white odd:bg-gray-50">
	                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
	                            <a href={format!("/view/{}", key.key)}>
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

                    }) }
                </tbody>
            </table>
        </div>
    }
}
