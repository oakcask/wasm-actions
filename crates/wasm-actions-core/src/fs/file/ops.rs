use js_sys::Uint8Array;
use wasm_actions_futures::JoinHandle;
use wasm_actions_node_sys::fs::{self, FileHandle, WriteOption};
use wasm_bindgen::JsValue;
use std::io::Result;

use crate::{error::Error, fs::file::File};

fn translate_error(js: JsValue) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other, // really this is appropriate?
        Error::from(js)
    )
}

pub(super) fn open(path: &str, flags: &str, mode: u32) -> JoinHandle<Result<File>> {
    wasm_actions_futures::from_promise(
        fs::open3(path, flags, mode),
        move |fd| Ok(File::new(FileHandle::from(fd))),
        move |e| Err(translate_error(e)),
    )
}

pub(super) fn write(fd: &FileHandle, buf: &[u8]) -> JoinHandle<Result<()>> {
    // we need to copy the slice because Promise returned by write2 lives longer than stack reference.
    let buf = Uint8Array::new_from_slice(buf);
    let buf = JsValue::from(buf);
    let promise = fd.write2(&buf, WriteOption::default());
    wasm_actions_futures::from_promise(
        promise, 
        move |_| Ok(()),
        move |e| Err(translate_error(e)))
}

pub(super) fn flush(fd: &FileHandle) -> JoinHandle<Result<()>> {
    let promise = fd.sync();
    wasm_actions_futures::from_promise(
        promise, 
        move |_| Ok(()),
        move |e| Err(translate_error(e)))   
}

pub(super) fn close(fd: &FileHandle) -> JoinHandle<Result<()>> {
    let promise = fd.close();
    wasm_actions_futures::from_promise(
        promise, 
        move |_| Ok(()),
        move |e| Err(translate_error(e)))   
}