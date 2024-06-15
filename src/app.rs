use crate::challenge::ChallengeComp;
use crate::footer::FooterComp;
use crate::points::PointsComp;
use crate::telegram;
use crate::version::VersionComp;
use crate::wallet::WalletComp;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let address = use_state(|| String::new());

    use_effect(move || {
        telegram::ready();
    });

    let on_address = {
        let address_clone = address.clone();
        Callback::from(move |address: String| {
            log::info!("Address: {}", address);
            address_clone.set(address);
        })
    };

    html! {
        <div>
            <h1>{"Konnektoren"}</h1>
            <VersionComp />
            <WalletComp on_address={on_address} />
            <PointsComp />
            <ChallengeComp id={"konnektoren-1"} address={(*address).clone()} />
            <FooterComp />
        </div>
    }
}
