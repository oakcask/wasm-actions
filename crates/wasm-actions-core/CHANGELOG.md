# Changelog

## [0.8.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-core-v0.7.0...wasm-actions-core-v0.8.0) (2025-12-01)


### ⚠ BREAKING CHANGES

* move Node library bindings to node-sys ([#90](https://github.com/oakcask/wasm-actions/issues/90))

### Code Refactoring

* move Node library bindings to node-sys ([#90](https://github.com/oakcask/wasm-actions/issues/90)) ([c228303](https://github.com/oakcask/wasm-actions/commit/c228303207b1720e2b4e9c3ab024e003f8f84cd8))

## [0.7.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-core-v0.6.0...wasm-actions-core-v0.7.0) (2025-11-30)


### ⚠ BREAKING CHANGES

* fs::open_append, fs::create_exclusive and apromise::Promise are removed from wasm-actions-core.

### Features

* add read_to_string ([#86](https://github.com/oakcask/wasm-actions/issues/86)) ([e5e91aa](https://github.com/oakcask/wasm-actions/commit/e5e91aa12559610f5c6168a10dc054e48647f1e0))
* File implements AsyncRead ([#85](https://github.com/oakcask/wasm-actions/issues/85)) ([ebbe13a](https://github.com/oakcask/wasm-actions/commit/ebbe13a961483fdf586cfc626aa984546ec91652))
* implement File and write operation ([#80](https://github.com/oakcask/wasm-actions/issues/80)) ([6434863](https://github.com/oakcask/wasm-actions/commit/643486381e53d58fa8433adfdd65ebdfc07a9e00))
* support conversion JsError to wasm-actions-core Error ([#82](https://github.com/oakcask/wasm-actions/issues/82)) ([32da971](https://github.com/oakcask/wasm-actions/commit/32da971f84657c322dd44a09783dbfe478057b21))


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#74](https://github.com/oakcask/wasm-actions/issues/74)) ([366535d](https://github.com/oakcask/wasm-actions/commit/366535d747e9dc47390ef7e98003f42268475dbd))

## [0.2.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-core-v0.1.1...wasm-actions-core-v0.2.0) (2025-11-22)


### ⚠ BREAKING CHANGES

* `env::temp_dir()` changed to return Node's `os.tmpdir()` instead.

### Features

* add clear_env which setup the test environment ([#30](https://github.com/oakcask/wasm-actions/issues/30)) ([7a4d018](https://github.com/oakcask/wasm-actions/commit/7a4d018d46f5f9f8f0a050fa0f8f4924dcc70202))
* add env::remove_var() and env::iter() ([#27](https://github.com/oakcask/wasm-actions/issues/27)) ([1cf5688](https://github.com/oakcask/wasm-actions/commit/1cf56881d0ed884c1424e009be2d0deff4a39565))
* add env::runner_temp_dir() ([#29](https://github.com/oakcask/wasm-actions/issues/29)) ([e8bde77](https://github.com/oakcask/wasm-actions/commit/e8bde77300b07ab112704f06a05122dc3c3e8b66))


### Bug Fixes

* address std::io::Write reimported error ([#31](https://github.com/oakcask/wasm-actions/issues/31)) ([cb73f5f](https://github.com/oakcask/wasm-actions/commit/cb73f5f14645df9edc146d00ffc35daa41df5a97))

## [0.1.1](https://github.com/oakcask/wasm-actions/compare/wasm-actions-core-v0.1.0...wasm-actions-core-v0.1.1) (2025-11-14)


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#16](https://github.com/oakcask/wasm-actions/issues/16)) ([cac474a](https://github.com/oakcask/wasm-actions/commit/cac474ab184e78716067db14edd6d9060469ad02))

## 0.1.0 (2025-11-13)


### Features

* add wasm-actions ([7093775](https://github.com/oakcask/wasm-actions/commit/70937758a0d8002c3dcca8e86a69f4086d8f0987))
