use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{js_sys::Promise, JsFuture};
use yew::prelude::*;

#[wasm_bindgen(module = "/src/js/confetti.js")]
extern "C" {
    fn fireConfetti();
}

#[wasm_bindgen(module = "/src/js/claim.js")]
extern "C" {
    fn claim(address: &str, amount: u32) -> Promise;
}

#[derive(Properties, PartialEq)]
pub struct ClaimCompProps {
    pub address: String,
    pub amount: u32,
}

enum ClaimState {
    Initial,
    Claiming,
    Done(String),
    Error(String),
}

#[function_component(ClaimComp)]
pub fn claim_comp(props: &ClaimCompProps) -> Html {
    let address = props.address.clone();
    let amount = props.amount;
    let claim_state = use_state(|| ClaimState::Initial);

    let on_claim = {
        let address = address.clone();
        let claim_state = claim_state.clone();

        Callback::from(move |_| {
            let address = address.clone();
            if address.is_empty() {
                claim_state.set(ClaimState::Error("Address is empty".to_string()));
                return;
            }

            log::info!("Claiming rewards {} {}", address, amount);
            fireConfetti();
            claim_state.set(ClaimState::Claiming);
            let claim_state = claim_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match JsFuture::from(claim(&address, amount)).await {
                    Ok(result) => {
                        let explorer_url = result.as_string().unwrap_or_default();
                        log::info!("Claim successful: {}", explorer_url);
                        claim_state.set(ClaimState::Done(explorer_url));
                    }
                    Err(e) => {
                        let error_message = format!("{:?}", e);
                        log::error!("Claim failed: {}", error_message);
                        claim_state.set(ClaimState::Error(error_message));
                    }
                }
            });
        })
    };

    let address_comp = {
        if address.is_empty() {
            html! { <p>{ "No address, connect wallet" }</p> }
        } else {
            html! {
                <span class="address">
                    <span class="symbol">{"🦄"}</span>
                    {format!("{}", address)}
                </span>
            }
        }
    };

    html! {
        <div class="claim">
            <h2>{ "Claim your rewards here" }</h2>
            { address_comp }
            {
                match &*claim_state {
                    ClaimState::Initial => html! {
                        <button class="claim-button" onclick={on_claim}>
                            <i class="fas fa-gift"></i>
                            {" Claim"}
                        </button>
                    },
                    ClaimState::Claiming => html! {
                        <button class="claim-button" disabled=true>
                            <i class="loader"></i>
                            {" Claiming..."}
                        </button>
                    },
                    ClaimState::Done(url) => html! {
                        <div>
                            <p>{ "Claim successful!" }</p>
                            <a href={url.clone()} target="_blank">{ "View on Explorer" }</a>
                        </div>
                    },
                    ClaimState::Error(e) => html! {
                        <div class="claim-container">
                            <button class="claim-button" onclick={on_claim}>
                                <i class="fas fa-gift"></i>
                                {" Claim"}
                            </button>
                            <div class="claim-error">
                                <p>{ "Claim failed!" }</p>
                                <p>{ e.clone() }</p>
                            </div>
                        </div>
                    },
                }
            }
        </div>
    }
}
