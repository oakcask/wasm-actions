use js_sys::Promise;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "WriteStream")]
    pub type WriteStream;

    #[wasm_bindgen(method, js_name = "write")]
    pub fn write2(this: &WriteStream, chunk: &JsValue, encoding: &JsValue) -> Promise;
}
