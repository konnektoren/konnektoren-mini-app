use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/telegram.js")]
extern "C" {
    #[wasm_bindgen(js_name = tma_ready)]
    pub fn ready();

    #[wasm_bindgen(js_name = tma_version)]
    pub fn version() -> String;

    #[wasm_bindgen(js_name = tma_set_header_color)]
    pub fn set_header_color(color: &str);

    #[wasm_bindgen(catch, js_name = tma_set_item)]
    pub async fn set_item(key: &str, value: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = tma_get_item)]
    pub async fn get_item(key: &str) -> Result<JsValue, JsValue>;
}
