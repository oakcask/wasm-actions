pub mod prelude {
    pub use wasm_actions_prelude::*;
}

#[cfg(feature = "derive")]
pub mod derive {
    pub use wasm_actions_derive::*;
    pub use wasm_actions_prelude::derive::*;
    // used by wasm-actions-derive macro
    #[doc(hidden)]
    pub use wasm_bindgen::prelude::wasm_bindgen;
    // used by wasm-actions-derive macro
    #[doc(hidden)]
    pub use wasm_bindgen::JsValue;
    // used by wasm-actions-derive macro
    #[doc(hidden)]
    pub use serde::Deserialize;
    #[doc(hidden)]
    pub use serde::Serialize;
    #[doc(hidden)]
    pub use wasm_bindgen::JsError;
    // used by wasm-actions-derive macro
    #[doc(hidden)]
    pub use serde_json;
}

#[cfg(feature = "testing")]
pub mod testing {
    pub use wasm_actions_prelude::testing::*;
}

pub mod futures {
    pub use wasm_actions_futures::*;
}
