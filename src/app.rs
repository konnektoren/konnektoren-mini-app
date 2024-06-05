use crate::challenge::ChallengeComp;
use crate::footer::Footer;
use crate::telegram;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    use_effect(move || {
        telegram::ready();
    });

    html! {
        <div>
            <h1>{"Konnektoren"}</h1>
            <ChallengeComp id={"konnektoren-1"} />
            <Footer />
        </div>
    }
}
