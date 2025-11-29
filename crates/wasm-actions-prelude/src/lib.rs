pub mod env;
mod output;
pub use output::save_state;
pub use output::set_output;
pub use wasm_actions_core::error::Error;
pub mod console;

pub mod fs {
    pub use wasm_actions_core::fs::*;
}
pub mod io {
    pub use wasm_actions_core::io::*;
}

#[cfg(feature = "derive")]
pub mod derive;

#[cfg(feature = "testing")]
pub mod testing;

#[macro_export]
macro_rules! get_input {
    ($name:expr) => {
        if let Some(value) = $crate::env::var(wasm_actions_macro::input_var!($name)) {
            Some(value)
        } else if let Some(value) =
            $crate::env::var(wasm_actions_macro::input_var_underscore!($name))
        {
            Some(value)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! get_state {
    ($name:expr) => {
        if let Some(value) = $crate::env::var(wasm_actions_macro::state_var!($name)) {
            Some(value)
        } else if let Some(value) =
            $crate::env::var(wasm_actions_macro::state_var_underscore!($name))
        {
            Some(value)
        } else {
            None
        }
    };
}

pub fn add_mask(value: &str) {
    console::log!("::add-mask::{}", value);
}
