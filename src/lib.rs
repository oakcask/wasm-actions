pub mod env;
mod output;
pub use output::save_state;
pub use output::set_output;
use wasm_actions_core::log;

#[macro_export]
macro_rules! get_input {
    ($name:expr) => {
        if let Some(value) = $crate::env::var(wasm_actions_macro::input_var!($name)) {
            Some(value)
        } else if let Some(value) = $crate::env::var(wasm_actions_macro::input_var_underscore!($name)) {
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
        } else if let Some(value) = $crate::env::var(wasm_actions_macro::state_var_underscore!($name)) {
            Some(value)
        } else {
            None
        }
    };
}

pub fn add_mask(value: &str) {
    log!("::add-mask::{}", value);
}
