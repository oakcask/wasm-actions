use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "node:events")]
extern "C" {
    #[wasm_bindgen(js_name = "EventEmitter")]
    pub type EventEmitter;

    #[wasm_bindgen(method)]
    pub fn on(this: &EventEmitter, event_name: &str, listener: Function);
}
