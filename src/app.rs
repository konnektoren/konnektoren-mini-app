use crate::challenge::ChallengeComp;
use crate::footer::Footer;
use crate::telegram;
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
            <WalletComp {on_address} />
            <ChallengeComp id={"konnektoren-1"} address={(*address).clone()} />
            <Footer />
        </div>
    }
}
