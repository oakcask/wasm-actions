use crate::io::StaticWriteStream;
use wasm_actions_node_sys::process::STDOUT;

pub fn stdout() -> StaticWriteStream {
    StaticWriteStream::new(&STDOUT)
}
