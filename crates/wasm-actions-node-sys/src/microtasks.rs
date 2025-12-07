use wasm_bindgen::prelude::{Closure, wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "queueMicrotask")]
    pub fn queue_microtask(clo: &Closure<dyn FnMut() -> ()>);
}
