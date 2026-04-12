use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use tempfile::tempdir;
use wasm_actions_build::generate_recommended;
use yaml_rust::YamlLoader;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn write_manifest(dir: &Path, package_name: &str, lib_path: Option<&str>) {
    let mut manifest = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "2024"
"#
    );
    if let Some(lib_path) = lib_path {
        manifest.push_str(&format!(
            r#"

[lib]
path = "{lib_path}"
"#
        ));
    }
    fs::write(dir.join("Cargo.toml"), manifest).unwrap();
}

fn with_manifest_dir<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
    let _guard = env_lock().lock().unwrap();
    let original_dir = env::current_dir().unwrap();
    let original_manifest_dir = env::var_os("CARGO_MANIFEST_DIR");

    env::set_current_dir(dir).unwrap();
    unsafe {
        env::set_var("CARGO_MANIFEST_DIR", dir);
    }

    let result = f();

    env::set_current_dir(original_dir).unwrap();
    match original_manifest_dir {
        Some(value) => unsafe {
            env::set_var("CARGO_MANIFEST_DIR", value);
        },
        None => unsafe {
            env::remove_var("CARGO_MANIFEST_DIR");
        },
    }

    result
}

fn parse_action_yaml(dir: &Path) -> yaml_rust::Yaml {
    let output = fs::read_to_string(dir.join("action.yaml")).unwrap();
    YamlLoader::load_from_str(&output).unwrap().remove(0)
}

#[test]
fn generate_recommended_uses_default_src_lib() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    write_manifest(dir.path(), "example-action", None);
    fs::write(
        dir.path().join("src/lib.rs"),
        r#"
            #[wasm_action(name = "Example", description = "Example action")]
            struct ExampleAction {
                #[input(name = "greeting", required = true)]
                greeting: String,
                #[output(name = "message", description = "Final message")]
                message: String,
            }
        "#,
    )
    .unwrap();

    with_manifest_dir(dir.path(), || {
        generate_recommended().unwrap();
    });

    let yaml = parse_action_yaml(dir.path());
    assert_eq!(yaml["name"].as_str(), Some("Example"));
    assert_eq!(yaml["inputs"]["greeting"]["required"].as_bool(), Some(true));
    assert_eq!(
        fs::read_to_string(dir.path().join("index.cjs")).unwrap(),
        "require(\"./pkg/example_action.js\").start()\n"
    );
}

#[test]
fn generate_recommended_respects_manifest_lib_path() {
    let dir = tempdir().unwrap();
    let lib_path = PathBuf::from("action/main.rs");
    fs::create_dir_all(dir.path().join("action")).unwrap();
    write_manifest(dir.path(), "custom-action", Some("action/main.rs"));
    fs::write(
        dir.path().join(&lib_path),
        r#"
            #[wasm_action(name = "Custom", description = "Custom action")]
            struct CustomAction {
                #[output(name = "message", description = "Final message")]
                message: String,
            }
        "#,
    )
    .unwrap();

    with_manifest_dir(dir.path(), || {
        generate_recommended().unwrap();
    });

    let yaml = parse_action_yaml(dir.path());
    assert_eq!(yaml["name"].as_str(), Some("Custom"));
    assert_eq!(
        yaml["outputs"]["message"]["description"].as_str(),
        Some("Final message")
    );
    assert_eq!(
        fs::read_to_string(dir.path().join("index.cjs")).unwrap(),
        "require(\"./pkg/custom_action.js\").start()\n"
    );
}
