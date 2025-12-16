use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(module = "node:buffer")]
extern "C" {
    /// https://nodejs.org/api/buffer.html#buffer
    #[wasm_bindgen(js_name = "Buffer")]
    pub type Buffer;

    /// https://nodejs.org/api/buffer.html#static-method-bufferisbufferobj
    #[wasm_bindgen(js_name = "isBuffer", static_method_of = Buffer)]
    pub fn is_buffer(obj: &JsValue) -> bool;
}
