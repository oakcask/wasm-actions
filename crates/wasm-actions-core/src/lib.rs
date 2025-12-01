pub mod error;
pub mod fs;
pub mod io;
pub mod os;
pub mod process;

#[cfg(feature = "crypto")]
pub mod crypto {
    pub use wasm_actions_node_sys::crypto::*;
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
