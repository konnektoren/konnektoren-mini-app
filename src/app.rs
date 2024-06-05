use crate::challenge::ChallengeComp;
use crate::telegram;
use yew::prelude::*;

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
            <ChallengeComp id={"konnektoren-1"} />
        </div>
    }
}
