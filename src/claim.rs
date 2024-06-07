use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{js_sys, JsFuture};
use yew::prelude::*;

#[wasm_bindgen(module = "/src/js/confetti.js")]
extern "C" {
    fn fireConfetti();
}

#[wasm_bindgen(module = "/src/js/claim.js")]
extern "C" {
    fn claim(address: &str, amount: u32) -> js_sys::Promise;
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
    let loading = use_state(|| false);

    let on_claim = {
        let address = address.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            log::info!("Claiming rewards {} {}", address, amount);
            fireConfetti();
            loading.set(true);
            let loading = loading.clone();
            let address = address.clone();
            wasm_bindgen_futures::spawn_local({
                let address = address.clone();
                let loading = loading.clone();
                async move {
                    match JsFuture::from(claim(&address, amount)).await {
                        Ok(_) => log::info!("Claim successful"),
                        Err(e) => log::error!("Claim failed: {:?}", e),
                    }
                    loading.set(false);
                }
            });
        })
    };

    html! {
        <div class="claim">
            <h2>{ "Claim your rewards here" }</h2>
            <p>{ format!("Address: {}", props.address) }</p>
            <button class="claim-button" onclick={on_claim} disabled={*loading}>
                if *loading {
                    <i class="loader"></i>
                    {" claiming..."}
                } else {
                    <i class="fas fa-gift"></i>
                    {" Claim"}
                }
            </button>
        </div>
    }
}
