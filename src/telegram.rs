use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function ready() { return Telegram.WebApp.ready(); }")]
extern "C" {
    pub fn ready();
}

#[wasm_bindgen(
    inline_js = "export function set_header_color(color) { return Telegram.WebApp.setHeaderColor(color); }"
)]
extern "C" {
    pub fn set_header_color(color: &str);
}
