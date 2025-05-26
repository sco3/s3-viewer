use crate::models::keyinfo::KeyInfo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyItemProps {
    pub key_info: KeyInfo,
}

#[function_component(KeyItem)]
pub fn key_item(props: &KeyItemProps) -> Html {
    let key = &props.key_info;
    html! {
        <tr>
            <td>{ &key.key }</td>
            <td>{ &key.last_modified }</td>
            <td>{ key.size }</td>
        </tr>
    }
}
