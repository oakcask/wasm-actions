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
    #[allow(dead_code)]
    offset: Integer,
    #[wasm_bindgen(js_name = "length")]
    #[allow(dead_code)]
    length: Option<Integer>,
    #[wasm_bindgen(js_name = "position")]
    #[allow(dead_code)]
    position: Option<Integer>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct CreateWriteStreamOption {
    #[wasm_bindgen(js_name = "encoding")]
    #[allow(dead_code)]
    encoding: Option<String>,
    #[wasm_bindgen(js_name = "autoClose")]
    #[allow(dead_code)]
    auto_close: Option<bool>,
    #[wasm_bindgen(js_name = "emitClose")]
    #[allow(dead_code)]
    emit_close: Option<bool>,
    #[wasm_bindgen(js_name = "start")]
    #[allow(dead_code)]
    start: Option<Integer>,
    #[wasm_bindgen(js_name = "highWaterMark")]
    #[allow(dead_code)]
    high_water_mark: Option<Integer>,
    #[wasm_bindgen(js_name = "flush")]
    #[allow(dead_code)]
    flush: bool,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct CreateReadStreamOption {
    #[wasm_bindgen(js_name = "encoding")]
    #[allow(dead_code)]
    encoding: Option<String>,
    #[wasm_bindgen(js_name = "autoClose")]
    #[allow(dead_code)]
    auto_close: Option<bool>,
    #[wasm_bindgen(js_name = "emitClose")]
    #[allow(dead_code)]
    emit_close: Option<bool>,
    #[wasm_bindgen(js_name = "start")]
    #[allow(dead_code)]
    start: Option<Integer>,
    #[wasm_bindgen(js_name = "end")]
    #[allow(dead_code)]
    end: Option<Integer>,
    #[wasm_bindgen(js_name = "highWaterMark")]
    #[allow(dead_code)]
    high_water_mark: Option<Integer>,
    #[wasm_bindgen(js_name = "signal")]
    #[allow(dead_code)]
    signal: bool,
}
