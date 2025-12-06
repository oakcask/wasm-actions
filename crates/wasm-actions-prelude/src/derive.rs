use std::str::FromStr;

use wasm_actions_futures::{JoinHandle, JsValue, JsError, spawn_microtask};

use crate::Error;

pub trait ActionInput {
    fn parse() -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ActionOutput {
    fn parse() -> Result<Option<Self>, Error>
    where
        Self: Sized;
    #[allow(async_fn_in_trait)]
    async fn save(self) -> Result<(), Error>;
}

pub trait Action<I: ActionInput, O: ActionOutput> {
    fn parse_input() -> Result<I, Error> {
        I::parse()
    }

    fn parse_state() -> Result<Option<O>, Error> {
        O::parse()
    }

    fn start() -> JoinHandle<Result<JsValue, JsError>> {
        spawn_microtask((async || {
            let input = Self::parse_input()?;
            if let Some(state) = Self::parse_state()? {
                Self::post(input, state).await?
            } else {
                let output = Self::main(input).await?;
                output.save().await?;
            }
            Ok(JsValue::UNDEFINED)
        })())
    }

    #[allow(async_fn_in_trait)]
    async fn main(input: I) -> Result<O, Error>;

    #[allow(async_fn_in_trait)]
    async fn post(_input: I, _state: O) -> Result<(), Error> {
        Ok(())
    }
}

pub trait ParseInput
where
    Self: Sized,
{
    fn parse(s: String) -> Result<Self, Error>;
}

impl<T> ParseInput for T
where
    T: FromStr + Sized,
    <T as FromStr>::Err: std::error::Error,
{
    fn parse(s: String) -> Result<T, Error> {
        s.as_str().parse().map_err(|e| Error::new(e))
    }
}

pub trait StringifyOutput {
    fn stringify(self) -> String;
}

impl<T> StringifyOutput for T
where
    T: Into<String> + Sized,
{
    fn stringify(self) -> String {
        self.into()
    }
}
