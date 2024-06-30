use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
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

#[wasm_bindgen(module = "/src/js/claim.js")]
extern "C" {
    fn claim_v2(address: &str, amount: u32) -> Promise;
}

#[wasm_bindgen(module = "/src/js/wallet.js")]
extern "C" {
    fn sendRawTransaction(raw_tx: &str, destination: &str) -> Promise;
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

#[allow(dead_code)]
enum ClaimVersion {
    V1,
    V2,
}

#[derive(Deserialize)]
struct RawTransaction {
    success: bool,
    raw_transaction: String,
    destination: String,
}

#[function_component(ClaimComp)]
pub fn claim_comp(props: &ClaimCompProps) -> Html {
    let address = props.address.clone();
    let amount = props.amount;
    let claim_state = use_state(|| ClaimState::Initial);
    const CLAIM_VERSION: ClaimVersion = ClaimVersion::V1;

    let on_claim: Callback<MouseEvent> = {
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
                match CLAIM_VERSION {
                    ClaimVersion::V1 => match JsFuture::from(claim(&address, amount)).await {
                        Ok(result) => {
                            log::info!("Claim successful: {:?}", result);
                            let explorer_url = result.as_string().unwrap_or_default();
                            log::info!("Claim successful: {}", explorer_url);
                            claim_state.set(ClaimState::Done(explorer_url));
                        }
                        Err(e) => {
                            let error_message = format!("{:?}", e);
                            log::error!("Claim failed: {}", error_message);
                            claim_state.set(ClaimState::Error(error_message));
                        }
                    },
                    ClaimVersion::V2 => match JsFuture::from(claim_v2(&address, amount)).await {
                        Ok(result) => match result.into_serde::<RawTransaction>() {
                            Ok(raw_tx) => {
                                if raw_tx.success {
                                    match JsFuture::from(sendRawTransaction(
                                        &raw_tx.raw_transaction,
                                        &raw_tx.destination,
                                    ))
                                    .await
                                    {
                                        Ok(_) => {
                                            log::info!("Transaction sent successfully");
                                            claim_state.set(ClaimState::Done(
                                                "Transaction sent successfully".to_string(),
                                            ));
                                        }
                                        Err(e) => {
                                            let error_message = format!("{:?}", e);
                                            log::error!("Transaction failed: {}", error_message);
                                            claim_state.set(ClaimState::Error(error_message));
                                        }
                                    }
                                } else {
                                    let error_message = "Failed to get raw transaction".to_string();
                                    log::error!("{}", error_message);
                                    claim_state.set(ClaimState::Error(error_message));
                                }
                            }
                            Err(e) => {
                                let error_message = format!("{:?}", e);
                                log::error!("Claim failed: {}", error_message);
                                claim_state.set(ClaimState::Error(error_message));
                            }
                        },
                        Err(e) => {
                            let error_message = format!("{:?}", e);
                            log::error!("Claim failed: {}", error_message);
                            claim_state.set(ClaimState::Error(error_message));
                        }
                    },
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
