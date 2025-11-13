use wasm_bindgen::prelude::wasm_bindgen;
use crate::{error::Error, io::WriteStream, promise::Promise};

#[wasm_bindgen(module = "node:fs/promises")]
extern "C" {
    #[wasm_bindgen(js_name = "open")]
    fn open2(path: &str, flags: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = "FileHandle")]
    #[derive(Clone)]
    type FileHandle;

    #[wasm_bindgen(method, js_name = "createWriteStream")]
    fn create_write_stream(this: &FileHandle, options: CreateWriteStreamOption) -> WriteStream;
}

#[wasm_bindgen]
struct CreateWriteStreamOption {
    // encoding
    #[wasm_bindgen(js_name = "autoClose")]
    #[allow(dead_code)]
    auto_close: bool,
    #[wasm_bindgen(js_name = "emitClose")]
    #[allow(dead_code)]
    emit_close: bool,
    // start
    // highWaterMark
    #[wasm_bindgen(js_name = "flush")]
    #[allow(dead_code)]
    flush: bool
}

impl Default for CreateWriteStreamOption {
    fn default() -> Self {
        Self {
            auto_close: true,
            emit_close: true,
            flush: false,
        }
    }
}

pub async fn open_append(name: &str) -> Result<WriteStream, Error> {
    let fd = open2(name, "a");
    let fd = Promise::new(fd, |js| Ok(FileHandle::from(js)));

    match fd.await {
        Ok(fd) => {
                let opts = CreateWriteStreamOption {
                flush: true,
                ..Default::default()
            };
            Ok(fd.create_write_stream(opts))                                
        }
        Err(e) => Err(e),
    }
}
