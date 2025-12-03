use proc_macro2::{Span, TokenStream};
use quote::{TokenStreamExt, quote};
use syn::{DeriveInput, Error, Ident};
use wasm_actions_parse::{InputAttr, InputSource, OutputAttr, OutputName};

pub(crate) fn start_fn(input: DeriveInput) -> Result<TokenStream, Error> {
    let ident = input.ident;
    // HACK: using #[wasm_bindgen] via wasm_bindgen_futures will generate
    // error complain about unlinked crate named `wasm_bindgen_futures`.
    // This provide small help for users to know that wasm-bindgen-futures
    // is needed as dependency.
    // But... it is painful so we might have to return a Promise directly.
    //
    // see wasm-actions.
    Ok(quote! {
        #[wasm_actions::derive::wasm_bindgen]
        pub fn start() -> wasm_actions::futures::Promise {
            let j: wasm_actions::futures::JoinHandle<
                Result<wasm_actions::derive::JsValue, wasm_actions::derive::JsError>
            > = wasm_actions::futures::spawn_microtask(
                    (async || {
                        use wasm_actions::derive::Action;
                        let input = #ident::parse_input()?;
                        if let Some(state) = #ident::parse_state()? {
                            #ident::post(input, state).await?
                        } else {
                            let output = #ident::main(input).await?;
                            use wasm_actions::derive::ActionOutput;
                            output.save().await?;
                        }
                        Ok(wasm_actions::derive::JsValue::UNDEFINED)
                    })()
                );
            j.into()
        }
    })
}

pub(crate) struct InputField {
    pub(crate) span: Span,
    pub(crate) field: Ident,
    pub(crate) attrs: Vec<InputAttr>,
}

impl InputField {
    pub(crate) fn input_source<'a>(&'a self) -> Result<InputSource<'a>, Error> {
        InputSource::try_from(&self.attrs).map_err(|e| Error::new(self.span, e))
    }
}

pub(crate) fn action_input_impl(
    struct_name: Ident,
    fields: Vec<InputField>,
) -> Result<TokenStream, Error> {
    let mut ts = TokenStream::new();
    for f in fields {
        let tokens = action_input_field_init(f)?;
        ts.append_all(tokens);
    }
    let out = quote! {
    impl wasm_actions::derive::ActionInput for #struct_name {
        fn parse() -> Result<Self, wasm_actions::prelude::Error> {
            Ok(Self {
                #ts
            })
        }
      }
    };
    Ok(out)
}

fn action_input_field_init(field: InputField) -> Result<TokenStream, Error> {
    let source = field.input_source()?;
    let field = field.field.clone();
    let code = match source {
        InputSource::Input(name) => {
            quote! {
                #field : (wasm_actions::derive::ParseInput::parse(
                    wasm_actions::prelude::get_input!(#name).ok_or_else(||
                        wasm_actions::prelude::Error::from(std::format!("{} missing", #name)))?)?),
            }
        }
        InputSource::Env(env) => {
            quote! {
                #field : (wasm_actions::derive::ParseInput::parse(
                    wasm_actions::prelude::env::var(#env).ok_or_else(||
                        wasm_actions::prelude::Error::from(std::format!("${} missing", #env)))?)?),
            }
        }
        InputSource::InputThenEnv { input: name, env } => {
            quote! {
                #field : (wasm_actions::derive::ParseInput::parse(
                    wasm_actions::prelude::get_input!(#name).or_else(|_|
                        wasm_actions::prelude::env::var(#env)
                    ).ok_or_else(|| wasm_actions::prelude::Error::from("either {} or ${} missing"))?
                )?),
            }
        }
    };
    Ok(code)
}

pub(crate) struct OutputField {
    pub(crate) field: Ident,
    pub(crate) attrs: Vec<OutputAttr>,
}

pub(crate) fn action_output_impl(
    struct_name: Ident,
    fields: Vec<OutputField>,
) -> Result<TokenStream, Error> {
    let mut ts = TokenStream::new();
    for f in fields {
        if let Some(tokens) = action_output_set_output(f)? {
            ts.append_all(tokens);
        }
    }

    let code = quote! {
        impl wasm_actions::derive::ActionOutput for #struct_name {
            fn parse() -> Result<Option<Self>, wasm_actions::prelude::Error> {
                if let Some(state) = wasm_actions::prelude::get_state!("wasm_actions") {
                    Ok(Some(serde_json::from_str(&state).map_err(Error::new)?))
                } else {
                    Ok(None)
                }
            }

            async fn save(self) -> Result<(), wasm_actions::prelude::Error> {
                let json = serde_json::to_string(&self).map_err(wasm_actions::prelude::Error::new)?;
                wasm_actions::prelude::save_state("wasm_actions", &json).await?;
                #ts
                Ok(())
            }
        }
    };
    Ok(code)
}

fn action_output_set_output(field: OutputField) -> Result<Option<TokenStream>, Error> {
    if let Some(name) = OutputName::try_from(&field.attrs) {
        let struct_field = field.field;
        Ok(Some(quote! {
            let #struct_field = wasm_actions::derive::StringifyOutput::stringify(self.#struct_field);
            wasm_actions::prelude::set_output(#name, &#struct_field).await?;
        }))
    } else {
        Ok(None)
    }
}
