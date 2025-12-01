use wasm_actions_macro::input_var;

const EXPECTED: &str = "INPUT_FOO_BAR-BAZ_";
const GOT: &str = input_var!("Foo bar-baZ ");

fn main() {
    assert_eq!(GOT, EXPECTED);
}
