use wasm_actions_core::process;

pub fn var(name: &str) -> Option<String> {
    process::get_env(name)
}

pub fn set_var(name: &str, value: &str) {
    process::set_env(name, value);
}
