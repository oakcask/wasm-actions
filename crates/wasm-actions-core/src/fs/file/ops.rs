use js_sys::{Reflect, Uint8Array};
use tokio::io::ReadBuf;
use std::{cmp::min, io::Result};
use wasm_actions_futures::JoinHandle;
use wasm_actions_node_sys::{Integer, fs::{self, FileHandle, WriteOption}};
use wasm_bindgen::{JsValue, convert::TryFromJsValue};

use crate::{error::Error, fs::file::File};

fn translate_error(js: JsValue) -> std::io::Error {
    std::io::Error::other(Error::from(js))
}

pub(super) fn open(path: &str, flags: &str, mode: u32) -> JoinHandle<Result<File>> {
    wasm_actions_futures::from_promise(
        fs::open3(path, flags, mode),
        move |fd| Ok(File::new(FileHandle::from(fd))),
        move |e| Err(translate_error(e)),
    )
}

pub(super) fn write(fd: &FileHandle, buf: &[u8]) -> JoinHandle<Result<usize>> {
    fn parse_write(js: JsValue) -> Result<usize> {
        let bytes_written= Reflect::get(&js, &JsValue::from_str("bytesWritten"))
            .map_err(translate_error)?;
        let bytes_written = bytes_written.as_f64().ok_or_else(
            || std::io::Error::new(std::io::ErrorKind::Other, Error::from("bytesWritten is not number")))?;
        Integer::from_f64_lossy(bytes_written)
            .try_into()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, Error::from(e)))
    }

    // we need to copy the slice because Promise returned by write2 lives longer than stack reference.
    let buf = Uint8Array::new_from_slice(buf);
    let buf = JsValue::from(buf);
    let promise = fd.write2(&buf, WriteOption::default());
    wasm_actions_futures::from_promise(promise, 
        parse_write,
     move |e| Err(translate_error(e)))
}

pub(super) fn flush(fd: &FileHandle) -> JoinHandle<Result<()>> {
    let promise = fd.sync();
    wasm_actions_futures::from_promise(promise, move |_| Ok(()), move |e| Err(translate_error(e)))
}

pub(super) fn close(fd: &FileHandle) -> JoinHandle<Result<()>> {
    let promise = fd.close();
    wasm_actions_futures::from_promise(promise, move |_| Ok(()), move |e| Err(translate_error(e)))
}



/// https://nodejs.org/api/fs.html#filehandlereadbuffer-options
pub(super) struct ReadResult {
    bytes_read: Integer,
    buffer: Uint8Array,
}

impl ReadResult {
    fn from_js(js: JsValue) -> Result<ReadResult> {
        let bytes_read = Reflect::get(&js, &JsValue::from_str("bytesRead"))
            .map_err(translate_error)?;
        let bytes_read = bytes_read.as_f64().ok_or_else(
            || std::io::Error::new(std::io::ErrorKind::Other, Error::from("bytesRead is not number")))?;
        let buffer = Reflect::get(&js, &JsValue::from_str("buffer"))
            .map_err(translate_error)?;
        let buffer = Uint8Array::try_from_js_value(buffer).map_err(translate_error)?;
        Ok(Self {
            bytes_read: Integer::from_f64_lossy(bytes_read),
            buffer
        })
    }

    pub fn copy_to(&self, buf: &mut ReadBuf) {
        let size: usize = self.bytes_read.try_into().unwrap();
        assert!(buf.remaining() >= size);
        // Safety: buffer.copy_to_uninit invocation guarantees 
        // that `size` bytes in unfilled region get initialized.
        let ptr = unsafe { &mut buf.unfilled_mut()[..size] };
        let buffer = self.buffer.slice(0, size.try_into().unwrap());
        buffer.copy_to_uninit(ptr); // FIXME
        // Safety: buffer.copy_to_uninit invocation guarantees
        // that `size` bytes in unfilled region get initialized.
        unsafe {
            buf.assume_init(size);
            buf.advance(size);
        }
    }
}

pub(super) fn read(fd: &FileHandle, size: usize) -> JoinHandle<Result<ReadResult>> {
    let size = min(u32::MAX as usize, size) as u32;
    let buffer = Uint8Array::new_with_length(size);
    let buffer = JsValue::from(buffer);
    let promise = fd.read1(&buffer);
    wasm_actions_futures::from_promise(promise,
    ReadResult::from_js,
        move |e| {
            Err(translate_error(e))
        })
}
