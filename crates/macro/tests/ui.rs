#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("ui-tests/pass/*.rs");
}

