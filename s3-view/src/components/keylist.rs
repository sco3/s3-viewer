use crate::components::keyitem::KeyItem;
use crate::models::keyinfo::KeyInfo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyListProps {
    pub keys: Vec<KeyInfo>,
}

#[function_component(KeyList)]
pub fn key_list(props: &KeyListProps) -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <th>{ "Key" }</th>
                    <th>{ "Last Modified" }</th>
                    <th>{ "Size (bytes)" }</th>
                </tr>
            </thead>
            <tbody>
                { for props.keys.iter().map(|key| html! {
                    <KeyItem key={key.key.clone()} key_info={key.clone()} />
                }) }
            </tbody>
        </table>
    }
}
