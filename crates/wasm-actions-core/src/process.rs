use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::io::{WriteStream, StaticWriteStream};

#[wasm_bindgen(module = "node:process")]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = "stdout")]
    static PROCESS_STDOUT: WriteStream;

    #[wasm_bindgen(thread_local_v2, js_name = "env")]
    static ENV: Object;
}

pub fn stdout() -> StaticWriteStream {
    StaticWriteStream::new(&PROCESS_STDOUT)
}

pub fn get_env(key: &str) -> Option<String> {
    let key = JsValue::from_str(key);
    get_env_by_jsvalue(&key)
}

fn get_env_by_jsvalue(key: &JsValue) -> Option<String> {
    let value= ENV.with(move |env| {Reflect::get(env, &key) });

    if let Ok(value) = value {
        value.as_string()
    } else {
        None
    }   
}

pub fn set_env(key: &str, value: &str) {
    let key = JsValue::from_str(key);
    let value = JsValue::from_str(value);

    ENV.with(move |env| {
        Reflect::set(env, &key, &value)
    }).unwrap();
}

pub fn delete_env(key: &str) {
    let key = JsValue::from_str(key);
    let _ = ENV.with(move |env| {
        Reflect::delete_property(env, &key)
    });
}

pub struct EnvIterator {
    keys: Array,
    next_index: u32,
    last_index: u32,
}

impl EnvIterator {
    pub fn new() -> Self {
        let keys = ENV.with(move |env| {
            Object::keys(env)
        });
        let last_index = keys.length();
        Self {
            keys,
            next_index: 0,
            last_index
        }
    }
}

impl Iterator for EnvIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_index >= self.last_index {
                break
            }
            let key = self.keys.get(self.next_index);
            let key_s = key.as_string().expect("key of process.env is expected to be a string");
            self.next_index += 1;

            // skip missing key.
            // ENV can be mutated while iterating over keys.
            // And we don't want to copy all the values in environment variables at initialization of EnvIterator. 
            if let Some(entry) = get_env_by_jsvalue(&key).map(|val| (key_s, val)) {
                return Some(entry)
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_env() {
        set_env("TEST_KEY", "TEST_VALUE");
        assert_eq!(get_env("TEST_KEY"), Some("TEST_VALUE".to_string()));
        assert_eq!(EnvIterator::new().find(|(k, _v)| k == "TEST_KEY"), Some((String::from("TEST_KEY"), String::from("TEST_VALUE"))));
        delete_env("TEST_KEY");
        assert_eq!(get_env("TEST_KEY"), None);
        assert_eq!(EnvIterator::new().find(|(k, _v)| k == "TEST_KEY"), None);
    }
}
