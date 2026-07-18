use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(module = "node:fs/promises")]
extern "C" {
    #[wasm_bindgen(js_name = "FileHandle")]
    pub type FileHandle;

    #[wasm_bindgen(js_name = "open")]
    pub fn open3(path: &str, flags: &str, mode: u32) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "close")]
    pub fn close(this: &FileHandle) -> js_sys::Promise;

    /// https://nodejs.org/api/fs.html#filehandlewritebuffer-options
    #[wasm_bindgen(method, js_name = "write")]
    pub fn write1(this: &FileHandle, buffer: &JsValue) -> js_sys::Promise;

    /// https://nodejs.org/api/fs.html#filehandlesync
    #[wasm_bindgen(method, js_name = "sync")]
    pub fn sync(this: &FileHandle) -> js_sys::Promise;

    /// https://nodejs.org/api/fs.html#filehandlereadbuffer-options
    #[wasm_bindgen(method, js_name = "read")]
    pub fn read1(this: &FileHandle, buffer: &JsValue) -> js_sys::Promise;
}
