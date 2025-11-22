use log::{debug, error, info, warn};
use wasm_bindgen_test::wasm_bindgen_test;


#[wasm_bindgen_test]
pub fn test() {
    let _ = wasm_actions_log::init();
    
    error!(file = "foo.rs", line = 42; "fix this error.");
    warn!("this is a warning");
    info!("This is an info message");
    debug!("This is a debug message");
}