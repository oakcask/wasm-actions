use std::{
    env,
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

use wasm_actions_parse::{InputAttr, OutputAttr, ParseFieldsNamed, WasmActionAttr};
use yaml_rust::{Yaml, YamlEmitter, yaml};

/// Generates metadata YAML and entrypoint script in
/// recommended configuration.
pub fn generate_recommended(cargo_crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut metadata = File::create("action.yaml")?;
        let metadata_path = PathBuf::from_str("src/lib.rs")?;
        generate_metadata_yaml(&mut metadata, &metadata_path)?;
        println!("cargo::rerun-if-changed=src/lib.rs");
    }
    {
        let index_cjs = PathBuf::from_str("index.cjs")?;
        let mut index_cjs = File::create(&index_cjs)?;
        generate_index_cjs(&mut index_cjs, cargo_crate_name)
    }
}

fn generate_metadata_yaml<T: std::io::Write>(
    w: &mut T,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let y = build_metadata_yaml(path)?;
    {
        let mut w = W(w);
        let mut emitter = YamlEmitter::new(&mut w);
        emitter.dump(&y)?;
    }
    writeln!(w)?;
    Ok(())
}

fn generate_index_cjs<T: std::io::Write>(
    w: &mut T,
    crate_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("./pkg/{}.js", crate_name);
    // escape JS string
    let path = serde_json::to_string(&path)?;
    writeln!(w, "require({path}).start()")?;
    Ok(())
}

fn build_metadata_yaml(path: &Path) -> Result<Yaml, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let tree = syn::parse_file(&content)?;

    let mut metadata = yaml::Hash::new();
    let mut inputs = yaml::Hash::new();
    let mut outputs = yaml::Hash::new();

    for item in tree.items {
        match item {
            syn::Item::Struct(item_struct) => {
                let wasm_action_attrs = WasmActionAttr::parse_attributes(&item_struct.attrs)?;
                for a in wasm_action_attrs {
                    match a {
                        WasmActionAttr::Name(s) => {
                            metadata.insert(
                                Yaml::String(String::from("name")),
                                Yaml::String(s.value()),
                            );
                        }
                        WasmActionAttr::Description(s) => {
                            metadata.insert(
                                Yaml::String(String::from("description")),
                                Yaml::String(s.value()),
                            );
                        }
                    }
                }
                match item_struct.fields {
                    syn::Fields::Named(fields_named) => {
                        let input_fields = InputAttr::parse_fields_named(fields_named.clone())?;
                        for f in input_fields {
                            let mut name = None;
                            let mut input_attrs = yaml::Hash::new();
                            for attr in f.attrs {
                                match attr {
                                    InputAttr::Name(lit_str) => {
                                        name = Some(Yaml::String(lit_str.value()))
                                    }
                                    InputAttr::Env(_) => {
                                        // ignore
                                    }
                                    InputAttr::Required(lit_bool) => {
                                        input_attrs.insert(
                                            Yaml::String(String::from("required")),
                                            Yaml::Boolean(lit_bool.value()),
                                        );
                                    }
                                    InputAttr::Description(lit_str) => {
                                        input_attrs.insert(
                                            Yaml::String(String::from("description")),
                                            Yaml::String(lit_str.value()),
                                        );
                                    }
                                    InputAttr::Default(lit_str) => {
                                        input_attrs.insert(
                                            Yaml::String(String::from("deefault")),
                                            Yaml::String(lit_str.value()),
                                        );
                                    }
                                }
                            }
                            if let Some(name) = name {
                                inputs.insert(name, Yaml::Hash(input_attrs));
                            }
                        }
                        let output_fields = OutputAttr::parse_fields_named(fields_named)?;
                        for f in output_fields {
                            let mut name = None;
                            let mut output_attrs = yaml::Hash::new();
                            for attr in f.attrs {
                                match attr {
                                    OutputAttr::Name(s) => name = Some(Yaml::String(s.value())),
                                    OutputAttr::Description(s) => {
                                        output_attrs.insert(
                                            Yaml::String(String::from("description")),
                                            Yaml::String(s.value()),
                                        );
                                    }
                                }
                            }
                            if let Some(name) = name {
                                outputs.insert(name, Yaml::Hash(output_attrs));
                            }
                        }
                    }
                    _ => {
                        // ignore
                    }
                }
            }
            _ => {
                // ignore
            }
        }
    }

    metadata.insert(Yaml::String(String::from("inputs")), Yaml::Hash(inputs));
    metadata.insert(Yaml::String(String::from("outputs")), Yaml::Hash(outputs));

    let mut runs = yaml::Hash::new();
    runs.insert(
        Yaml::String(String::from("using")),
        Yaml::String(String::from("node24")),
    );
    runs.insert(
        Yaml::String(String::from("main")),
        Yaml::String(String::from("index.cjs")),
    );
    runs.insert(
        Yaml::String(String::from("post")),
        Yaml::String(String::from("index.cjs")),
    );
    metadata.insert(Yaml::String(String::from("runs")), Yaml::Hash(runs));

    Ok(Yaml::Hash(metadata))
}

struct W<'a, T: std::io::Write>(&'a mut T);

impl<'a, T: std::io::Write> std::fmt::Write for W<'a, T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(&mut self.0, "{}", s).map_err(|_| std::fmt::Error)
    }
}
