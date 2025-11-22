#[cfg(feature = "proc-macro")]
use proc_macro2::Span;
#[cfg(feature = "proc-macro")]
use syn::spanned::Spanned;
use syn::{
    Attribute, Error, Expr, ExprLit, FieldsNamed, Ident, Lit, LitBool, LitStr, Token, parse::Parse,
    punctuated::Punctuated,
};

pub struct FieldWithAttributes<T> {
    pub ident: Ident,
    pub attrs: Vec<T>,
    #[cfg(feature = "proc-macro")]
    pub span: Span,
}

pub trait ParseFieldsNamed
where
    Self: Sized,
{
    // provided
    fn parse_fields_named(
        fields: FieldsNamed,
    ) -> Result<Vec<FieldWithAttributes<Self>>, syn::Error> {
        let mut results = Vec::with_capacity(fields.named.len());
        for f in fields.named.into_iter() {
            #[cfg(feature = "proc-macro")]
            let span = f.span();
            if let Some(ident) = f.ident {
                let attrs = Self::parse_attributes(&f.attrs)?;
                results.push(FieldWithAttributes {
                    ident,
                    attrs,
                    #[cfg(feature = "proc-macro")]
                    span,
                });
            }
        }
        Ok(results)
    }

    fn parse_attributes(attr: &[Attribute]) -> Result<Vec<Self>, syn::Error>;
}

pub enum InputSource<'a> {
    Input(&'a LitStr),
    Env(&'a LitStr),
    InputThenEnv { input: &'a LitStr, env: &'a LitStr },
}

impl<'a> InputSource<'a> {
    pub fn try_from(attrs: &'a [InputAttr]) -> Result<Self, &'static str> {
        let mut input = None;
        let mut env = None;
        for a in attrs {
            match a {
                InputAttr::Name(s) => input = Some(s),
                InputAttr::Env(s) => env = Some(s),
                _ => {}
            }
        }

        match (input, env) {
            (None, None) => Err("#[input] expectes at least one name or env"),
            (None, Some(e)) => Ok(Self::Env(e)),
            (Some(i), None) => Ok(Self::Input(i)),
            (Some(i), Some(e)) => Ok(Self::InputThenEnv { input: i, env: e }),
        }
    }
}

pub enum InputAttr {
    Name(LitStr),
    Env(LitStr),
    Required(LitBool),
    Description(LitStr),
    Default(LitStr),
}

#[cfg(feature = "proc-macro")]
pub struct OutputName<'a>(&'a LitStr);

#[cfg(feature = "proc-macro")]
impl<'a> OutputName<'a> {
    pub fn try_from(attrs: &'a [OutputAttr]) -> Option<Self> {
        for a in attrs {
            match a {
                OutputAttr::Name(lit_str) => return Some(Self(lit_str)),
                _ => {
                    // next
                }
            }
        }
        None
    }
}

#[cfg(feature = "proc-macro")]
impl<'a> quote::ToTokens for OutputName<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub enum OutputAttr {
    Name(LitStr),
    Description(LitStr),
}

impl ParseFieldsNamed for InputAttr {
    fn parse_attributes(attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error> {
        let mut input_attrs = Vec::new();

        for a in attrs {
            if a.path().is_ident("input") {
                let kvs =
                    a.parse_args_with(Punctuated::<InputAttr, Token![,]>::parse_terminated)?;
                for kv in kvs {
                    input_attrs.push(kv)
                }
            }
        }

        Ok(input_attrs)
    }
}

impl Parse for InputAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let key_s = key.to_string();
        let t_assign = input.parse::<Token![=]>()?;
        let value = match input.parse::<Expr>() {
            Ok(expr) => expr,
            Err(_) => {
                return Err(Error::new(
                    t_assign.span,
                    "expected literal expression after `=`",
                ));
            }
        };
        match key_s.as_str() {
            "default" => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(InputAttr::Default(s)),
                _ => Err(Error::new(
                    t_assign.span,
                    "expected literal string after `=`".to_string(),
                )),
            },
            "description" => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(InputAttr::Description(s)),
                _ => Err(Error::new(
                    t_assign.span,
                    "expected literal string after `=`".to_string(),
                )),
            },
            "env" => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(InputAttr::Env(s)),
                _ => Err(Error::new(
                    t_assign.span,
                    "expected literal string after `=`".to_string(),
                )),
            },
            "name" => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Ok(InputAttr::Name(s)),
                _ => Err(Error::new(
                    t_assign.span,
                    "expected literal string after `=`".to_string(),
                )),
            },
            "required" => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(s), ..
                }) => Ok(InputAttr::Required(s)),
                _ => Err(Error::new(
                    t_assign.span,
                    "expected literal bool after `=`".to_string(),
                )),
            },
            unknown => Err(Error::new(
                key.span(),
                format!("#[input] cannot accept `{unknown}`"),
            )),
        }
    }
}

impl ParseFieldsNamed for OutputAttr {
    fn parse_attributes(attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error> {
        let mut input_attrs = Vec::new();

        for a in attrs {
            if a.path().is_ident("output") {
                let kvs =
                    a.parse_args_with(Punctuated::<OutputAttr, Token![,]>::parse_terminated)?;
                for kv in kvs {
                    input_attrs.push(kv)
                }
            }
        }

        Ok(input_attrs)
    }
}

impl Parse for OutputAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let key_s = key.to_string();

        let t_assign = input.parse::<Token![=]>()?;
        let value = if input.peek(LitStr) {
            input.parse()?
        } else {
            return Err(Error::new(
                t_assign.span,
                "expected literal string after `=`",
            ));
        };
        match key_s.as_str() {
            "description" => Ok(OutputAttr::Description(value)),
            "name" => Ok(OutputAttr::Name(value)),
            unknown => Err(Error::new(
                key.span(),
                format!("#[output] cannot accept `{unknown}`"),
            )),
        }
    }
}

pub enum WasmActionAttr {
    Name(LitStr),
    Description(LitStr),
}

impl WasmActionAttr {
    pub fn parse_attributes(attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error> {
        let mut input_attrs = Vec::new();

        for a in attrs {
            if a.path().is_ident("wasm_action") {
                let kvs = a.parse_args_with(Punctuated::<Self, Token![,]>::parse_terminated)?;
                for kv in kvs {
                    input_attrs.push(kv)
                }
            }
        }

        Ok(input_attrs)
    }
}

impl Parse for WasmActionAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let key_s = key.to_string();
        let t_assign = input.parse::<Token![=]>()?;
        let value = if input.peek(LitStr) {
            input.parse()?
        } else {
            return Err(Error::new(
                t_assign.span,
                "expected literal string or expression after `=`",
            ));
        };
        match key_s.as_str() {
            "name" => Ok(WasmActionAttr::Name(value)),
            "description" => Ok(WasmActionAttr::Description(value)),
            unknown => Err(Error::new(
                key.span(),
                format!("#[wasm_action] cannot accept `{unknown}`"),
            )),
        }
    }
}
