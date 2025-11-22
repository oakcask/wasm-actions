# Changelog

## [0.3.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-prelude-v0.2.0...wasm-actions-prelude-v0.3.0) (2025-11-22)


### Features

* prelude exposes Error from wasm_actions_core ([#38](https://github.com/oakcask/wasm-actions/issues/38)) ([76ee534](https://github.com/oakcask/wasm-actions/commit/76ee534e83d3a81cd6450401016dec8513075c86))

## [0.2.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-prelude-v0.1.0...wasm-actions-prelude-v0.2.0) (2025-11-22)


### ⚠ BREAKING CHANGES

* `env::temp_dir()` changed to return Node's `os.tmpdir()` instead.

### Features

* add clear_env which setup the test environment ([#30](https://github.com/oakcask/wasm-actions/issues/30)) ([7a4d018](https://github.com/oakcask/wasm-actions/commit/7a4d018d46f5f9f8f0a050fa0f8f4924dcc70202))
* add env::remove_var() and env::iter() ([#27](https://github.com/oakcask/wasm-actions/issues/27)) ([1cf5688](https://github.com/oakcask/wasm-actions/commit/1cf56881d0ed884c1424e009be2d0deff4a39565))
* add env::runner_temp_dir() ([#29](https://github.com/oakcask/wasm-actions/issues/29)) ([e8bde77](https://github.com/oakcask/wasm-actions/commit/e8bde77300b07ab112704f06a05122dc3c3e8b66))
* add env::temp_dir() ([#28](https://github.com/oakcask/wasm-actions/issues/28)) ([7a31e5a](https://github.com/oakcask/wasm-actions/commit/7a31e5a1f1ec6f4ce0efed1509bfa77193b584b8))
* add wasm-actions-derive ([#23](https://github.com/oakcask/wasm-actions/issues/23)) ([27cc4f6](https://github.com/oakcask/wasm-actions/commit/27cc4f66b976f7c7462f0322359b97c9e4a9b8ac))

## 0.1.0 (2025-11-22)


### ⚠ BREAKING CHANGES

* 

### Code Refactoring

* move wasm_actions::* to wasm_actions::prelude ([#21](https://github.com/oakcask/wasm-actions/issues/21)) ([489e51e](https://github.com/oakcask/wasm-actions/commit/489e51ef9e15a4af0ddf85a61ba69d8bbd45b414))
