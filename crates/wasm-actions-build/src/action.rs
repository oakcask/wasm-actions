use std::{error::Error, fs, path::Path};

use wasm_actions_parse::{InputAttr, OutputAttr, ParseFieldsNamed as _, WasmActionAttr};
use yaml_rust::{Yaml, YamlEmitter, yaml};

pub fn generate_metadata_yaml<T: std::io::Write>(
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

pub fn generate_index_cjs<T: std::io::Write>(
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
                                            Yaml::String(String::from("default")),
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

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;
    use yaml_rust::YamlLoader;

    use super::{generate_index_cjs, generate_metadata_yaml};

    fn write_source(dir: &Path, source: &str) -> std::path::PathBuf {
        let source_path = dir.join("action.rs");
        fs::write(&source_path, source).unwrap();
        source_path
    }

    fn parse_metadata(source: &str) -> yaml_rust::Yaml {
        let dir = tempdir().unwrap();
        let source_path = write_source(dir.path(), source);
        let mut output = Vec::new();
        generate_metadata_yaml(&mut output, &source_path).unwrap();

        let output = String::from_utf8(output).unwrap();
        YamlLoader::load_from_str(&output).unwrap().remove(0)
    }

    #[test]
    fn metadata_yaml_uses_default_key_for_input_defaults() {
        let yaml = parse_metadata(
            r#"
                #[wasm_action(name = "Example", description = "Example action")]
                struct ExampleAction {
                    #[input(name = "greeting", default = "hello world")]
                    greeting: String,
                }
            "#,
        );
        let greeting = &yaml["inputs"]["greeting"];

        assert_eq!(greeting["default"].as_str(), Some("hello world"));
    }

    #[test]
    fn metadata_yaml_covers_supported_attributes_and_runs_block() {
        let yaml = parse_metadata(
            r#"
                #[wasm_action(name = "Example", description = "Example action")]
                struct ExampleAction {
                    #[input(
                        name = "greeting",
                        env = "GREETING",
                        required = true,
                        description = "Greeting to use",
                        default = "hello"
                    )]
                    greeting: String,
                    #[output(name = "message", description = "Final message")]
                    message: String,
                }
            "#,
        );

        assert_eq!(yaml["name"].as_str(), Some("Example"));
        assert_eq!(yaml["description"].as_str(), Some("Example action"));
        assert_eq!(yaml["inputs"]["greeting"]["required"].as_bool(), Some(true));
        assert_eq!(
            yaml["inputs"]["greeting"]["description"].as_str(),
            Some("Greeting to use")
        );
        assert_eq!(yaml["inputs"]["greeting"]["default"].as_str(), Some("hello"));
        assert!(yaml["inputs"]["greeting"]["env"].is_badvalue());
        assert_eq!(
            yaml["outputs"]["message"]["description"].as_str(),
            Some("Final message")
        );
        assert_eq!(yaml["runs"]["using"].as_str(), Some("node24"));
        assert_eq!(yaml["runs"]["main"].as_str(), Some("index.cjs"));
        assert_eq!(yaml["runs"]["post"].as_str(), Some("index.cjs"));
    }

    #[test]
    fn metadata_yaml_skips_fields_without_name_attributes() {
        let yaml = parse_metadata(
            r#"
                struct ExampleAction {
                    #[input(env = "GREETING", description = "ignored without name")]
                    greeting: String,
                    #[output(description = "ignored without name")]
                    message: String,
                }
            "#,
        );

        assert!(yaml["inputs"]["greeting"].is_badvalue());
        assert!(yaml["outputs"]["message"].is_badvalue());
    }

    #[test]
    fn metadata_yaml_ignores_non_named_structs_and_non_struct_items() {
        let yaml = parse_metadata(
            r#"
                #[wasm_action(name = "Tuple", description = "ignored")]
                struct TupleAction(
                    #[input(name = "tuple")]
                    String
                );

                #[wasm_action(name = "Unit", description = "ignored")]
                struct UnitAction;

                #[wasm_action(name = "fn", description = "ignored")]
                fn helper() {}
            "#,
        );

        assert_eq!(yaml["name"].as_str(), Some("Unit"));
        assert_eq!(yaml["description"].as_str(), Some("ignored"));
        assert_eq!(yaml["inputs"].as_hash().unwrap().len(), 0);
        assert_eq!(yaml["outputs"].as_hash().unwrap().len(), 0);
    }

    #[test]
    fn metadata_yaml_combines_structs_and_last_top_level_metadata_wins() {
        let yaml = parse_metadata(
            r#"
                #[wasm_action(name = "First", description = "First action")]
                struct FirstAction {
                    #[input(name = "foo")]
                    foo: String,
                }

                #[wasm_action(name = "Second", description = "Second action")]
                struct SecondAction {
                    #[output(name = "bar", description = "Bar output")]
                    bar: String,
                }
            "#,
        );

        assert_eq!(yaml["name"].as_str(), Some("Second"));
        assert_eq!(yaml["description"].as_str(), Some("Second action"));
        assert!(yaml["inputs"]["foo"].as_hash().is_some());
        assert_eq!(
            yaml["outputs"]["bar"]["description"].as_str(),
            Some("Bar output")
        );
    }

    #[test]
    fn metadata_yaml_errors_on_invalid_input_key() {
        let dir = tempdir().unwrap();
        let source_path = write_source(
            dir.path(),
            r#"
                struct ExampleAction {
                    #[input(unknown = "value")]
                    greeting: String,
                }
            "#,
        );
        let mut output = Vec::new();

        let error = generate_metadata_yaml(&mut output, &source_path).unwrap_err();

        assert!(error.to_string().contains("#[input] cannot accept `unknown`"));
    }

    #[test]
    fn metadata_yaml_errors_on_invalid_output_key() {
        let dir = tempdir().unwrap();
        let source_path = write_source(
            dir.path(),
            r#"
                struct ExampleAction {
                    #[output(required = "yes")]
                    greeting: String,
                }
            "#,
        );
        let mut output = Vec::new();

        let error = generate_metadata_yaml(&mut output, &source_path).unwrap_err();

        assert!(error
            .to_string()
            .contains("#[output] cannot accept `required`"));
    }

    #[test]
    fn metadata_yaml_errors_on_invalid_wasm_action_key() {
        let dir = tempdir().unwrap();
        let source_path = write_source(
            dir.path(),
            r#"
                #[wasm_action(required = "yes")]
                struct ExampleAction;
            "#,
        );
        let mut output = Vec::new();

        let error = generate_metadata_yaml(&mut output, &source_path).unwrap_err();

        assert!(error
            .to_string()
            .contains("#[wasm_action] cannot accept `required`"));
    }

    #[test]
    fn metadata_yaml_errors_on_wrong_literal_type() {
        let dir = tempdir().unwrap();
        let source_path = write_source(
            dir.path(),
            r#"
                struct ExampleAction {
                    #[input(required = "yes")]
                    greeting: String,
                }
            "#,
        );
        let mut output = Vec::new();

        let error = generate_metadata_yaml(&mut output, &source_path).unwrap_err();

        assert!(error
            .to_string()
            .contains("expected literal bool after `=`"));
    }

    #[test]
    fn metadata_yaml_errors_on_invalid_rust_source() {
        let dir = tempdir().unwrap();
        let source_path = write_source(dir.path(), "struct ExampleAction {");
        let mut output = Vec::new();

        assert!(generate_metadata_yaml(&mut output, &source_path).is_err());
    }

    #[test]
    fn metadata_yaml_matches_expected_output_snapshot() {
        let dir = tempdir().unwrap();
        let source_path = write_source(
            dir.path(),
            r#"
                #[wasm_action(name = "Example", description = "Example action")]
                struct ExampleAction {
                    #[input(name = "greeting", required = true, description = "Greeting to use", default = "hello")]
                    greeting: String,
                    #[output(name = "message", description = "Final message")]
                    message: String,
                }
            "#,
        );
        let mut output = Vec::new();

        generate_metadata_yaml(&mut output, &source_path).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(
            output,
            "\
---
name: Example
description: Example action
inputs:
  greeting:
    required: true
    description: Greeting to use
    default: hello
outputs:
  message:
    description: Final message
runs:
  using: node24
  main: index.cjs
  post: index.cjs
"
        );
    }

    #[test]
    fn index_cjs_matches_expected_output() {
        let mut output = Vec::new();

        generate_index_cjs(&mut output, "my_crate").unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "require(\"./pkg/my_crate.js\").start()\n"
        );
    }

    #[test]
    fn index_cjs_escapes_special_characters() {
        let mut output = Vec::new();

        generate_index_cjs(&mut output, "crate\"name").unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "require(\"./pkg/crate\\\"name.js\").start()\n"
        );
    }
}
