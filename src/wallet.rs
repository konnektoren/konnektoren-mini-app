use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/js/wallet.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn connectWallet(on_connect: JsValue) -> Result<JsValue, JsValue>;
}

type WalletConnectCallback = Closure<dyn Fn(String, String)>;

#[derive(Properties, PartialEq)]
pub struct WalletCompProps {
    pub on_address: Callback<String>,
}

#[function_component(WalletComp)]
pub fn wallet(props: &WalletCompProps) -> Html {
    let address = use_state(|| "Not connected".to_string());
    let balance = use_state(|| "0".to_string());

    {
        let address = address.clone();
        let balance = balance.clone();
        let on_address = props.on_address.clone();
        use_effect(move || {
            let address = address.clone();
            let balance = balance.clone();
            let on_address = on_address.clone();
            let on_connect: WalletConnectCallback = {
                let address = address.clone();
                let balance = balance.clone();
                let on_address = on_address.clone();
                Closure::wrap(Box::new(move |addr: String, bal: String| {
                    log::info!("Connected in rs: {} {}", addr, bal);
                    address.set(addr.clone());
                    balance.set(bal);
                    on_address.emit(addr);
                }) as Box<dyn Fn(String, String)>)
            };

            let on_connect_js_value = JsValue::from(on_connect.as_ref());
            wasm_bindgen_futures::spawn_local(async move {
                match connectWallet(on_connect_js_value).await {
                    Ok(_) => {}
                    Err(e) => address.set(format!("Initialization Error: {:?}", e)),
                }
            });

            on_connect.forget();
            || ()
        });
    }

    html! {
        <div class="wallet">
            <div id="wallet-button" />
            <span class="balance">
                <span class="symbol">{"★"}</span>
                {format!("{}", *balance)}
            </span>
        </div>
    }
}
