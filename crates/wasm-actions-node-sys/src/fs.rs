use crate::Integer;
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
    pub fn write2(this: &FileHandle, buffer: &JsValue, options: WriteOption) -> js_sys::Promise;

    /// https://nodejs.org/api/fs.html#filehandlesync
    #[wasm_bindgen(method, js_name = "sync")]
    pub fn sync(this: &FileHandle) -> js_sys::Promise;

    /// https://nodejs.org/api/fs.html#filehandlereadbuffer-options
    #[wasm_bindgen(method, js_name = "read")]
    pub fn read1(this: &FileHandle, buffer: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WriteOption {
    #[wasm_bindgen(js_name = "offset")]
    pub offset: Integer,
    #[wasm_bindgen(js_name = "length")]
    pub length: Option<Integer>,
    #[wasm_bindgen(js_name = "position")]
    pub position: Option<Integer>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct CreateWriteStreamOption {
    #[wasm_bindgen(js_name = "encoding")]
    pub encoding: Option<String>,
    #[wasm_bindgen(js_name = "autoClose")]
    pub auto_close: Option<bool>,
    #[wasm_bindgen(js_name = "emitClose")]
    pub emit_close: Option<bool>,
    #[wasm_bindgen(js_name = "start")]
    pub start: Option<Integer>,
    #[wasm_bindgen(js_name = "highWaterMark")]
    pub high_water_mark: Option<Integer>,
    #[wasm_bindgen(js_name = "flush")]
    pub flush: bool,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct CreateReadStreamOption {
    #[wasm_bindgen(js_name = "encoding")]
    pub encoding: Option<String>,
    #[wasm_bindgen(js_name = "autoClose")]
    pub auto_close: Option<bool>,
    #[wasm_bindgen(js_name = "emitClose")]
    pub emit_close: Option<bool>,
    #[wasm_bindgen(js_name = "start")]
    pub start: Option<Integer>,
    #[wasm_bindgen(js_name = "end")]
    pub end: Option<Integer>,
    #[wasm_bindgen(js_name = "highWaterMark")]
    pub high_water_mark: Option<Integer>,
    #[wasm_bindgen(js_name = "signal")]
    pub signal: bool,
}
