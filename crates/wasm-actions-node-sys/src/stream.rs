use js_sys::Function;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::Integer;

#[wasm_bindgen(module = "node:stream")]
extern "C" {
    /// https://nodejs.org/api/stream.html#class-streamreadable
    #[wasm_bindgen]
    pub type Readable;
    #[wasm_bindgen(method, js_name = "read")]
    /// https://nodejs.org/api/stream.html#readablereadsize
    pub fn read1(this: &Readable, size: Integer) -> JsValue;
    /// https://nodejs.org/api/stream.html#class-streamreadable
    #[wasm_bindgen(method)]
    pub fn on(this: &Readable, event_name: &str, listener: Function);

    /// https://nodejs.org/api/stream.html#class-streamwritable
    #[wasm_bindgen]
    pub type Writable;
    /// https://nodejs.org/api/stream.html#writablewritechunk-encoding-callback
    #[wasm_bindgen(method, js_name = "write")]
    pub fn write2(this: &Writable, chunk: &JsValue, encoding: &JsValue) -> bool;
    /// https://nodejs.org/api/stream.html#writableendchunk-encoding-callback
    #[wasm_bindgen(method, js_name = "end")]
    pub fn end0(this: &Writable);
}
