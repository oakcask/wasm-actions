use wasm_actions::{env, get_state};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
pub fn test_get_state() {
    env::set_var("STATE_FOO_BAR", "42");

    assert_eq!(get_state!("foo-bar"), Some("42".to_string()));    
    assert_eq!(get_state!("foo_bar"), Some("42".to_string()));    
}
