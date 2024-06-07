use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/js/confetti.js")]
extern "C" {
    fn fireConfetti();
}

#[derive(Properties, PartialEq)]
pub struct ClaimCompProps {
    pub address: String,
    pub amount: u32,
}

#[function_component(ClaimComp)]
pub fn claim_comp(props: &ClaimCompProps) -> Html {
    let address = props.address.clone();
    let amount = props.amount;
    let on_claim = Callback::from(move |_| {
        log::info!("Claiming rewards {} {}", address, amount);
        fireConfetti();
    });

    html! {
        <div class="claim">
            <h2>{ "Claim your rewards here" }</h2>
            <p>{ format!("Address: {}", props.address) }</p>
            <button class="claim-button" onclick={on_claim}>
                <i class="fas fa-gift"></i>
                {" Claim"}
            </button>
        </div>
    }
}
