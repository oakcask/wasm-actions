use js_sys::Uint8Array;
use wasm_actions_node_sys::io::WriteStream;
use wasm_bindgen::{JsThreadLocal, JsValue};

pub use tokio::io::AsyncReadExt;
pub use tokio::io::AsyncWriteExt;

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
