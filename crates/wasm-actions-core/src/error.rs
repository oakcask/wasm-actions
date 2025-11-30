use std::fmt::Display;

use wasm_bindgen::{JsError, JsValue};

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error { message: value }
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error {
            message: format!("{:?}", value),
        }
    }
}

impl From<JsError> for Error {
    fn from(value: JsError) -> Self {
        let value: JsValue = value.into();
        value.into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new<E: std::error::Error>(e: E) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}