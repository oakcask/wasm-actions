use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Crypto;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = crypto)]
    static CRYPTO: Crypto;
}

pub fn random_uuid() -> String {
    CRYPTO.with(move |c| {
        c.random_uuid()
    })
}