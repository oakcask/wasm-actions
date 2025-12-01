pub mod env;
pub mod error;
pub mod fs;
pub mod io;
pub mod process;

#[cfg(feature = "crypto")]
pub mod crypto {
    pub use wasm_actions_node_sys::crypto::*;
}
pub mod os {
    pub use wasm_actions_node_sys::os::tmpdir;
}

#[macro_export]
macro_rules! log {
    () => {
        {
            use std::io::Write;
            let mut stdout = $crate::process::stdout();
            let _ = std::writeln!(&mut stdout);
        }
    };
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            let mut stdout = $crate::process::stdout();
            let _ = std::writeln!(&mut stdout, $($arg)*);
        }
    };
}
