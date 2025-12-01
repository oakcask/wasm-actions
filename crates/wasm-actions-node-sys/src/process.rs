use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "node:process")]
extern "C" {
    #[cfg(feature = "io")]
    #[wasm_bindgen(thread_local_v2, js_name = "stdout")]
    pub static STDOUT: crate::io::WriteStream;

    #[wasm_bindgen(thread_local_v2, js_name = "env")]
    pub static ENV: Object;
}
