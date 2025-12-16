use js_sys::Array;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::stream::{Readable, Writable};

#[wasm_bindgen(module = "node:child_process")]
extern "C" {
    /// https://nodejs.org/api/child_process.html#child-process
    #[wasm_bindgen(extends = crate::events::EventEmitter)]
    pub type ChildProcess;

    /// https://nodejs.org/api/child_process.html#child_processspawncommand-args-options
    #[wasm_bindgen(js_name = "spawn")]
    pub fn spawn(command: &str, args: Array, options: JsValue) -> ChildProcess;
    /// https://nodejs.org/api/child_process.html#subprocessstdin
    #[wasm_bindgen(js_name = "stdin", method, getter)]
    pub fn stdin(this: &ChildProcess) -> Option<Writable>;
    /// https://nodejs.org/api/child_process.html#subprocessstdout
    #[wasm_bindgen(js_name = "stdout", method, getter)]
    pub fn stdout(this: &ChildProcess) -> Option<Readable>;
    /// https://nodejs.org/api/child_process.html#subprocessstderr
    #[wasm_bindgen(js_name = "stderr", method, getter)]
    pub fn stderr(this: &ChildProcess) -> Option<Readable>;
}
