use crate::error::Error;
use std::future::Future;
use std::io::{ErrorKind, Result};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use wasm_actions_futures::JoinHandle;
use wasm_actions_node_sys::fs::FileHandle;
mod ops;

/// Provide interface alike std::fs::File.
pub struct File {
    handle: FileHandle,
    state: State,
    last_err: Option<std::io::Error>,
}

enum State {
    Idle,
    Busy(Operation),
}

enum Operation {
    Write(Pin<Box<JoinHandle<Result<()>>>>),
}

impl tokio::io::AsyncWrite for File {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize>> {
        let this = self.as_mut().get_mut();
        if let Some(e) = this.last_err.take() {
            return Poll::Ready(Err(e));
        }

        loop {
            match this.state {
                State::Idle => {
                    this.state =
                        State::Busy(Operation::Write(Box::pin(ops::write(&this.handle, buf))));
                    return Poll::Ready(Ok(buf.len()));
                }
                State::Busy(Operation::Write(ref mut op)) => {
                    let res = ready!(op.as_mut().poll(cx));
                    this.state = State::Idle;
                    res?;
                }
            }
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Result<()>> {
        let this = self.as_mut().get_mut();
        if let Some(e) = this.last_err.take() {
            return Poll::Ready(Err(e));
        }

        loop {
            match this.state {
                State::Idle => {
                    this.state = State::Busy(Operation::Write(Box::pin(ops::flush(&this.handle))));
                    return Poll::Ready(Ok(()));
                }
                State::Busy(Operation::Write(ref mut op)) => {
                    let res = ready!(op.as_mut().poll(cx));
                    this.state = State::Idle;
                    res?
                }
            }
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Result<()>> {
        let this = self.as_mut().get_mut();
        if let Some(e) = this.last_err.take() {
            return Poll::Ready(Err(e));
        }

        loop {
            match this.state {
                State::Idle => {
                    this.state = State::Busy(Operation::Write(Box::pin(ops::close(&this.handle))));
                    return Poll::Ready(Ok(()));
                }
                State::Busy(Operation::Write(ref mut op)) => {
                    let res = ready!(op.as_mut().poll(cx));
                    this.state = State::Idle;
                    res?
                }
            }
        }
    }
}

/// Provide interface alike std::fs::OpenOptions.
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    synchronous: bool,
    mode: u32,
}

macro_rules! file_option_setters {
    ( $ty:ident; $( $x:ident ), * ) => {
        $(
            pub fn $x(&mut self, $x: $ty) -> &mut Self {
                self.$x = $x;
                self
            }
        )*
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        Self {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            synchronous: false,
            mode: 0o666,
        }
    }

    file_option_setters!(bool;
        read,
        write,
        append,
        truncate,
        create,
        create_new,
        synchronous
    );

    file_option_setters!(u32; mode);

    /// Invoke fs.open
    pub async fn open<P: AsRef<str>>(&self, p: P) -> Result<File> {
        ops::open(p.as_ref(), self.as_flags()?, self.mode).await
    }

    /// Convert OpenOptions to Node style open-mode flags
    ///
    /// - `append` ignores `create`, `create_new`, `write`, and `truncate`. because "a" always create file if missing.
    /// - will fail if `create`, `create_new`, and `truncate` specified without `write` (like std::fs::OpenOptions).
    /// - will fail if `create`, `create_new`, and `truncate` specified with `synchronous`. because "ws" is not available.
    fn as_flags(&self) -> Result<&'static str> {
        fn err(mesg: &'static str) -> Result<&'static str> {
            Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                Error::from(mesg),
            ))
        }
        let truncate = self.create_new | self.create | self.truncate;
        if self.append {
            match (self.create_new, self.synchronous, self.read) {
                (true, true, _) => {
                    err("`create_new` and `synchronous` cannot be used simultaneously")
                }
                (true, false, true) => Ok("ax+"),
                (true, false, false) => Ok("ax"),
                (false, true, true) => Ok("as+"),
                (false, true, false) => Ok("as"),
                (false, false, true) => Ok("a+"),
                (false, false, false) => Ok("a"),
            }
        } else if truncate {
            match (self.write, self.synchronous, self.create_new, self.read) {
                (false, _, _, _) => {
                    err("`create_new`, `create`, and `truncate` require `write` to take effect")
                }
                (true, true, _, _) => {
                    err("`synchronous` cannot be used with `create_new`, `create`, and `truncate`")
                }
                (true, false, true, true) => Ok("wx+"),
                (true, false, true, false) => Ok("wx"),
                (true, false, false, true) => Ok("w+"),
                (true, false, false, false) => Ok("w"),
            }
        } else {
            match (self.read, self.synchronous, self.write) {
                (false, _, _) => err("either `read` or `write` require to open a file"),
                (true, true, true) => Ok("rs+"),
                (true, true, false) => Ok("rs"),
                (true, false, true) => Ok("r+"),
                (true, false, false) => Ok("r"),
            }
        }
    }
}

impl File {
    fn new(handle: FileHandle) -> Self {
        File {
            handle,
            state: State::Idle,
            last_err: None,
        }
    }

    /// Open a file in read-only mode.
    pub async fn open<P: AsRef<str>>(p: P) -> Result<Self> {
        OpenOptions::new().read(true).open(p).await
    }

    /// Open a file in write-only mode with truncating content.
    pub async fn create<P: AsRef<str>>(p: P) -> Result<Self> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(p)
            .await
    }

    /// Open a file in read-write mode, with truncating content;
    /// fail if already exists.
    pub async fn create_new<P: AsRef<str>>(p: P) -> Result<Self> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(p)
            .await
    }
}
