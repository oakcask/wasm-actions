use std::path::PathBuf;
use js_sys::{Array, Object, Reflect};
use wasm_actions_node_sys::{os, process::ENV};
use wasm_bindgen::JsValue;

pub fn var(name: &str) -> Option<String> {
    let key = JsValue::from_str(name);
    get_env_by_jsvalue(&key)
}

pub fn set_var(name: &str, value: &str) {
    let key = JsValue::from_str(name);
    let value = JsValue::from_str(value);

    ENV.with(move |env| Reflect::set(env, &key, &value))
        .unwrap();
}

pub fn remove_var(name: &str) {
    let key = JsValue::from_str(name);
    let _ = ENV.with(move |env| Reflect::delete_property(env, &key));
}

pub fn vars() -> EnvIterator {
    EnvIterator::new()
}

pub fn temp_dir() -> PathBuf {
    PathBuf::from(os::tmpdir())
}

pub fn runner_temp_dir() -> PathBuf {
    // https://docs.github.com/en/actions/reference/workflows-and-actions/variables
    let runner_temp = var("RUNNER_TEMP").expect("$RUNNER_TEMP is expected to be set");
    PathBuf::from(runner_temp)
}

fn get_env_by_jsvalue(key: &JsValue) -> Option<String> {
    let value = ENV.with(move |env| Reflect::get(env, key));

    if let Ok(value) = value {
        value.as_string()
    } else {
        None
    }
}

pub struct EnvIterator {
    keys: Array,
    next_index: u32,
    last_index: u32,
}

impl Default for EnvIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvIterator {
    pub(crate) fn new() -> Self {
        let keys = ENV.with(Object::keys);
        let last_index = keys.length();
        Self {
            keys,
            next_index: 0,
            last_index,
        }
    }
}

impl Iterator for EnvIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_index >= self.last_index {
                break;
            }
            let key = self.keys.get(self.next_index);
            let key_s = key
                .as_string()
                .expect("key of process.env is expected to be a string");
            self.next_index += 1;

            // skip missing key.
            // ENV can be mutated while iterating over keys.
            // And we don't want to copy all the values in environment variables at initialization of EnvIterator.
            if let Some(entry) = get_env_by_jsvalue(&key).map(|val| (key_s, val)) {
                return Some(entry);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::env;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_env() {
        env::set_var("TEST_KEY", "TEST_VALUE");
        assert_eq!(env::var("TEST_KEY"), Some("TEST_VALUE".to_string()));
        assert_eq!(
            env::vars().find(|(k, _v)| k == "TEST_KEY"),
            Some((String::from("TEST_KEY"), String::from("TEST_VALUE")))
        );
        env::remove_var("TEST_KEY");
        assert_eq!(env::var("TEST_KEY"), None);
        assert_eq!(env::vars().find(|(k, _v)| k == "TEST_KEY"), None);
    }
}
