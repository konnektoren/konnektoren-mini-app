use crate::route::{AddressParam, Route};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct RouletteProps {
    pub address: String,
}

#[function_component(Roulette)]
pub fn roulette(props: &RouletteProps) -> Html {
    let links = vec![
        ("articles-2", "der die das"),
        ("reflexivpronoun-2", "Reflexive Pronomen"),
        ("personal_pronouns-2", "Personal Pronomen"),
        ("konnektoren-2", "Konnektoren"),
    ];

    html! {
        <div class="roulette">
            { for links.iter().enumerate().map(|(i, (id, text))| html! {
                <div class={format!("slice slice-{}", i)}>
                    <Link<Route, AddressParam> to={Route::Challenge { id: id.to_string() }}
                        query={AddressParam { address: props.address.clone() }}>
                        <span class="link-text">{ text }</span>
                    </Link<Route, AddressParam>>
                </div>
            }) }
            <div class="center-circle">
            </div>
        </div>
    }
}
