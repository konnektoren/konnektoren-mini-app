use crate::api::profile::fetch_profile;
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
        let _ = telegram::ready()
            .map_err(|error| log::error!("Error initializing telegram: {:?}", error));

        wasm_bindgen_futures::spawn_local(async move {
            let id: String = telegram::get_user_id().unwrap_or("0".to_string());
            log::debug!("ID: {}", id);
            let username: String = telegram::get_user_name().unwrap_or("Anonymous".to_string());
            log::debug!("Username: {}", username);

            let profile = fetch_profile(id.to_string(), username)
                .await
                .expect("error loading profile");
            log::info!("Profile: {:?}", profile);
        });
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
