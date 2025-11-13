use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::io::{WriteStream, StaticWriteStream};

#[wasm_bindgen(module = "node:process")]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = "stdout")]
    static PROCESS_STDOUT: WriteStream;

    #[wasm_bindgen(thread_local_v2, js_name = "env")]
    static ENV: JsValue;
}

pub fn stdout() -> StaticWriteStream {
    StaticWriteStream::new(&PROCESS_STDOUT)
}

pub fn get_env(key: &str) -> Option<String> {
    let key = JsValue::from_str(key);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_env() {
        set_env("TEST_KEY", "TEST_VALUE");
        assert_eq!(get_env("TEST_KEY"), Some("TEST_VALUE".to_string()));
    }
}
