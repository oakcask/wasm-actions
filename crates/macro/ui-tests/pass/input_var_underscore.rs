use actions_rs_macro::input_var_underscore;

const EXPECTED: &str = "INPUT_FOO_BAR_BAZ_";
const GOT: &str = input_var_underscore!("Foo bar-baZ ");

fn main() {
    assert_eq!(GOT, EXPECTED);
}
