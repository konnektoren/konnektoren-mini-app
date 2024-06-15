use crate::telegram;
use yew::prelude::*;

#[function_component(VersionComp)]
pub fn version_comp() -> Html {
    let version = use_state(|| "loading".to_string());

    {
        let version = version.clone();
        use_effect_with(version.clone(), move |_| {
            let version = version.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api_version = telegram::version();
                log::info!("Version: {}", api_version);
                version.set(api_version);
            });
            || ()
        });
    }

    html! {
        <div class="version">
            <span class="value">{"Version: "}{(*version).to_string()}</span>
        </div>
    }
}
