use yew::prelude::*;
use crate::telegram;

#[function_component(App)]
pub fn app() -> Html {

    use_effect(move || {
        telegram::ready();
        telegram::set_header_color("red");
    });
    html! {
        <div>
            <h1>{"Konnektoren"}</h1>
            {"Visit "}<a href="https://konnektoren.help">{"Konnektoren.help"}</a>
        </div>
    }
}
