use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{"Konnektoren"}</h1>
            {"Visit "}<a href="https://konnektoren.help">{"Konnektoren.help"}</a>
        </div>
    }
}
