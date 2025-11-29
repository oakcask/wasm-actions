use std::sync::Mutex;

use async_once_cell::OnceCell;
use wasm_actions_core::{
    error::Error,
    fs::{File, OpenOptions},
};

use crate::env;

struct Port {
    name: &'static str,
    lock: Mutex<Option<Result<File, Error>>>,
    init: OnceCell<()>,
}

impl Port {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            lock: Mutex::new(None),
            init: OnceCell::new(),
        }
    }

    async fn open(&self) -> Result<File, Error> {
        let name =
            env::var(self.name).ok_or_else(|| Error::from(format!("${} unset", self.name)))?;
        OpenOptions::new()
            .append(true)
            .open(&name)
            .await
            .map_err(|e| Error::new(e))
    }

    async fn with<'a, T, F: FnOnce(&'a mut File) -> T>(&'a mut self, f: F) -> Result<T, Error> {
        self.init
            .get_or_try_init(async {
                let r = self.open().await;
                if let Ok(mut g) = self.lock.lock() {
                    *g = Some(r);
                    Ok(())
                } else {
                    Err(Error::from(format!(
                        "failed to acquiring lock for ${} file handle",
                        self.name
                    )))
                }
            })
            .await?;
        if let Ok(g) = self.lock.get_mut() {
            match g {
                Some(Ok(w)) => Ok(f(w)),
                Some(Err(e)) => Err(e.clone()),
                None => unreachable!(),
            }
        } else {
            Err(Error::from(format!(
                "failed to acquiring lock for ${} file handle",
                self.name
            )))
        }
    }
}

// Safety: Synchronized by Mutex and won't be available directly for the library user.
unsafe impl Sync for Port {}
// Safety: Synchronized by Mutex and won't be available directly for the library user.
unsafe impl Send for Port {}

static mut OUTPUT_PORT: Port = Port::new("GITHUB_OUTPUT");
static mut STATE_PORT: Port = Port::new("GITHUB_STATE");

pub async fn set_output(name: &str, value: &str) -> Result<(), Error> {
    use std::io::Write;
    let mut buf = Vec::<u8>::new();
    writeln!(&mut buf, "{}={}", name, value).map_err(Error::new)?;

    let result = unsafe {
        // Safety: mutable reference `w` only lives in the period of the Port's lock is taken.
        #[allow(static_mut_refs)]
        OUTPUT_PORT.with(|w| wasm_actions_core::io::AsyncWriteExt::write_all(w, &buf))
    };
    result.await.map(|_| ())
}

pub async fn save_state(name: &str, value: &str) -> Result<(), Error> {
    use std::io::Write;
    let mut buf = Vec::<u8>::new();
    writeln!(&mut buf, "{}={}", name, value).map_err(Error::new)?;

    let result = unsafe {
        // Safety: mutable reference `w` only lives in the period of the Port's lock is taken.
        #[allow(static_mut_refs)]
        STATE_PORT.with(|w| wasm_actions_core::io::AsyncWriteExt::write_all(w, &buf))
    };
    result.await.map(|_| ())
}
