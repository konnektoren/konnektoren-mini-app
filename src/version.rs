use crate::telegram;
use yew::prelude::*;

#[function_component(VersionComp)]
pub fn version() -> Html {
    let version = use_state(|| String::new());

    {
        let version = version.clone();
        use_effect(move || {
            let version = version.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api_version = telegram::version();
                version.set(api_version);
            });
        });
    }

    html! {
        <div class="version">
            <span class="value">{"Version: "}{(*version).to_string()}</span>
        </div>
    }
}
