use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "node:os")]
extern "C" {
    #[wasm_bindgen(js_name = "tmpdir")]
    pub fn tmpdir() -> String;
}
