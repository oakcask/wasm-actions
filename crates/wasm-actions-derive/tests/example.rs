use wasm_actions::derive::*;
use wasm_actions::prelude::macros::input_var;
use wasm_actions::prelude::*;
use wasm_actions::testing::*;
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_action(name = "example", description = "example action")]
struct Example;

#[derive(ActionInput)]
struct Input {
    #[input(name = "foo", required = true, description = "input parameter foo")]
    foo: String,
    #[input(env = "BAR")]
    bar: String,
}

#[derive(ActionOutput, serde::Serialize, serde::Deserialize)]
struct Output {
    #[output(name = "message", description = "message from action")]
    message: String,
}

impl Action<Input, Output> for Example {
    async fn main(input: Input) -> Result<Output, Error> {
        Ok(Output {
            message: format!("foo = {}, bar = {}", input.foo, input.bar),
        })
    }
}

#[wasm_bindgen_test]
async fn fail_if_called_without_input() -> Result<(), JsError> {
    let _guard = clear_env().await;

    if let Err(e) = start().await.map_err(|e| format!("{:?}", JsValue::from(e))) {
        assert!(e.starts_with("JsValue(Error: foo missing\n"))
    } else {
        panic!("unexpectedly succeeded")
    }
    Ok(())
}

#[wasm_bindgen_test]
async fn runs_main_if_inputs_are_filled() -> Result<(), JsError> {
    let _guard = clear_env().await;

    env::set_var(input_var!("foo"), "42");
    env::set_var("BAR", "4242");

    start().await?;

    let s = env::var("GITHUB_OUTPUT").unwrap();
    let s = fs::read_to_string(&s)
        .await
        .expect("read_to_string() failed");
    assert_eq!(&s, "message=foo = 42, bar = 4242\n");

    Ok(())
}
