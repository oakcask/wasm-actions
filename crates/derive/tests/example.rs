use wasm_actions_macro::input_var;
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_actions_derive::{ wasm_action, ActionInput, ActionOutput };
use wasm_actions_prelude::derive::Action;
use wasm_actions_core::{error::Error, process};

#[wasm_action(
    name = "example",
    description = "example action"
)]
struct Example;

#[derive(ActionInput)]
struct Input {
    #[input(
        name = "foo",
        required = true,
        description = "input parameter foo"
    )]
    foo: String,
    #[input(
        env = "BAR",
    )]
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
    // TODO: reset process.env to ensure the environment clean.
    if let Err(e) = start().await.map_err(|e| format!("{:?}", JsValue::from(e))) {
        assert!(e.starts_with("JsValue(Error: foo missing\n"))
    } else {
        unreachable!()
    }
    Ok(())
}

// TODO: enable this after we can reset process.env in test.
// #[wasm_bindgen_test]
#[allow(dead_code)]
async fn runs_main_if_inputs_are_filled() -> Result<(), JsError> {
    process::set_env(input_var!("foo"), "42");
    process::set_env("BAR", "4242");

    if let Err(e) = start().await.map_err(|e| format!("{:?}", JsValue::from(e))) {
        unreachable!("{e}")
    } else {
        assert!(true)
    }
    Ok(())
}
