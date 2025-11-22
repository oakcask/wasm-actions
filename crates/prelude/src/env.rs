use std::path::PathBuf;

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

pub fn temp_dir() -> PathBuf {
    // https://docs.github.com/en/actions/reference/workflows-and-actions/variables
    let runner_temp = var("RUNNER_TEMP").expect("$RUNNER_TEMP is expected to be set");
    PathBuf::from(runner_temp)
}
