use js_sys::Uint8Array;
use wasm_actions_node_sys::stream::Writable;
use wasm_bindgen::{JsThreadLocal, JsValue};

pub use tokio::io::AsyncReadExt;
pub use tokio::io::AsyncWriteExt;

pub struct StaticWriteStream {
    var: &'static JsThreadLocal<Writable>,
}

impl StaticWriteStream {
    pub(crate) fn new(var: &'static JsThreadLocal<Writable>) -> Self {
        StaticWriteStream { var }
    }
}

impl std::io::Write for StaticWriteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = buf.len();
        let buf = Uint8Array::from(buf);
        let buf = JsValue::from(buf);
        let encoding = JsValue::null();
        // TODO: handle flush
        let _ = self.var.with(move |this| this.write2(&buf, &encoding));
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
