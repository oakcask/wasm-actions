
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsThreadLocal, JsValue};

pub use tokio::io::AsyncWriteExt;

// TODO: move this out to node-sys
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "WriteStream")]
    pub(crate) type WriteStream;

    #[wasm_bindgen(method, js_name = "write")]
    pub(crate) fn write2(this: &WriteStream, chunk: &JsValue, encoding: &JsValue) -> Promise;
}

impl std::io::Write for WriteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = buf.len();
        let buf = Uint8Array::from(buf);
        let buf = JsValue::from(buf);
        let encoding = JsValue::null();
        // TODO: handle promise
        let _ = self.write2(&buf, &encoding);
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct StaticWriteStream {
    var: &'static JsThreadLocal<WriteStream>,
}

impl StaticWriteStream {
    pub(crate) fn new(var: &'static JsThreadLocal<WriteStream>) -> Self {
        StaticWriteStream { var }
    }
}

impl std::io::Write for StaticWriteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = buf.len();
        let buf = Uint8Array::from(buf);
        let buf = JsValue::from(buf);
        let encoding = JsValue::null();
        // TODO: handle promise
        let _ = self.var.with(move |this| this.write2(&buf, &encoding));
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
