use wasm_actions_core::process::{self, EnvIterator};

pub fn var(name: &str) -> Option<String> {
    process::get_env(name)
}

pub fn set_var(name: &str, value: &str) {
    process::set_env(name, value);
}

pub fn remove_var(name: &str) {
    process::delete_env(name);
}

pub fn vars() -> EnvIterator {
    EnvIterator::new()
}
