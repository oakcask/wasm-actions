use wasm_actions_prelude::{env, get_input};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
pub fn test_get_input() {
    env::set_var("INPUT_FOO_BAR", "42");

    assert_eq!(get_input!("foo-bar"), Some("42".to_string()));
    assert_eq!(get_input!("foo_bar"), Some("42".to_string()));
}
