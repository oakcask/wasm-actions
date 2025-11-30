pub mod prelude {
    pub use wasm_actions_prelude::*;
}

#[cfg(feature = "derive")]
pub mod derive {
    // HACK: using #[wasm_bindgen] via wasm_bindgen_futures will generate
    // error complain about unlinked crate named `wasm_bindgen_futures`.
    // This provide small help for users to know that wasm-bindgen-futures
    // is needed as dependency.
    // But... it is painful so we might have to return a Promise directly.
    #[doc(hidden)]
    pub use wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen;
    pub use wasm_actions_derive::*;
    pub use wasm_actions_prelude::derive::*;
}

#[cfg(feature = "testing")]
pub mod testing {
    pub use wasm_actions_prelude::testing::*;
}
