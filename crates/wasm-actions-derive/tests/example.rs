use wasm_actions::derive::{Action, ActionInput, ActionOutput, wasm_action};
use wasm_actions::futures::JoinHandle;
use wasm_actions::prelude::macros::input_var;
use wasm_actions::prelude::{env, fs};
use wasm_actions::testing::clear_env;
use wasm_bindgen::JsError;
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
    async fn main(input: Input) -> Result<Output, wasm_actions::prelude::Error> {
        Ok(Output {
            message: format!("foo = {}, bar = {}", input.foo, input.bar),
        })
    }
}

#[wasm_bindgen_test]
async fn fail_if_called_without_input() -> Result<(), JsError> {
    let _guard = clear_env().await;

    let fut = JoinHandle::from_promise(start(), move |_| Ok(()), move |e| Err(format!("{:?}", e)));

    if let Err(e) = fut.await {
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

    JoinHandle::from_promise(start(), move |_| Ok(()), move |_| Err(()))
        .await
        .unwrap();

    let s = env::var("GITHUB_OUTPUT").unwrap();
    let s = fs::read_to_string(&s)
        .await
        .expect("read_to_string() failed");
    assert_eq!(&s, "message=foo = 42, bar = 4242\n");

    Ok(())
}
