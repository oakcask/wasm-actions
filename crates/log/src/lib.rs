use log::SetLoggerError;

struct Logger;
static LOGGER: Logger = Logger;

struct ArgBuf(Vec<u8>);

impl ArgBuf {
    fn new() -> Self {
        ArgBuf(Vec::new())
    }

    fn take(self) -> String {
        unsafe { String::from_utf8_unchecked(self.0) }
    }
}

impl<'kvs> log::kv::VisitSource<'kvs> for ArgBuf {
    fn visit_pair(&mut self, key: log::kv::Key<'kvs>, value: log::kv::Value<'kvs>) -> Result<(), log::kv::Error> {
        use std::io::Write;
        let result = if self.0.is_empty() {
            write!(self.0, " {}={}", key, value)
        } else {
            write!(self.0, ",{}={}", key, value)
        };
        result.map_err(log::kv::Error::from)
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let level = match level {
                log::Level::Error => "error",
                log::Level::Warn => "warning",
                log::Level::Info => "notice",
                log::Level::Debug => "debug",
                log::Level::Trace => "debug",
            };
            let msg = record.args();
            let mut buf = ArgBuf::new();
            if record.key_values().visit(&mut buf).is_ok() {
                wasm_actions_core::log!("::{}{}::{}", level, buf.take(), msg);                
            } else {
                wasm_actions_core::log!("::{}::{}", level, msg);
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the logger.
/// 
/// ## Example
/// 
/// ```no_run
/// let _ = wasm_actions_log::init();
/// log::info!("this is notice message.");
/// log::error!(file = "foo.rs", line = 42; "fix this error.");
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}
