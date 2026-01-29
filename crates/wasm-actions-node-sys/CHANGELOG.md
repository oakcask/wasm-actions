# Changelog

## [0.3.1](https://github.com/oakcask/wasm-actions/compare/wasm-actions-node-sys-v0.3.0...wasm-actions-node-sys-v0.3.1) (2026-01-29)


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#173](https://github.com/oakcask/wasm-actions/issues/173)) ([e81ae53](https://github.com/oakcask/wasm-actions/commit/e81ae5328a0748fa3a48717288ccdd2bb59aa15c))

## [0.3.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-node-sys-v0.2.0...wasm-actions-node-sys-v0.3.0) (2026-01-04)


### ⚠ BREAKING CHANGES

* removes io::WritableStream from wasm-actions-node-sys.

### Features

* add child_process and relating interfaces ([#139](https://github.com/oakcask/wasm-actions/issues/139)) ([c95e839](https://github.com/oakcask/wasm-actions/commit/c95e839f445cc6788479af4dd9efa901a76b2dd0))
* add from_f64_clamping ([#133](https://github.com/oakcask/wasm-actions/issues/133)) ([3c9424f](https://github.com/oakcask/wasm-actions/commit/3c9424f90149db9869ef5eca46c927f38f13a11d))
* implement Display for Integer ([#138](https://github.com/oakcask/wasm-actions/issues/138)) ([3c7fc0e](https://github.com/oakcask/wasm-actions/commit/3c7fc0ed648bc6bec6dd62416d939562d4b5fae6))


### Bug Fixes

* fix wrongly typed stream API ([#124](https://github.com/oakcask/wasm-actions/issues/124)) ([c75495f](https://github.com/oakcask/wasm-actions/commit/c75495f3bb7285f007aa810b6cef876b6c106360))

## [0.2.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-node-sys-v0.1.0...wasm-actions-node-sys-v0.2.0) (2025-12-01)


### ⚠ BREAKING CHANGES

* move Node library bindings to node-sys ([#90](https://github.com/oakcask/wasm-actions/issues/90))

### Code Refactoring

* move Node library bindings to node-sys ([#90](https://github.com/oakcask/wasm-actions/issues/90)) ([c228303](https://github.com/oakcask/wasm-actions/commit/c228303207b1720e2b4e9c3ab024e003f8f84cd8))

## 0.1.0 (2025-11-30)


### ⚠ BREAKING CHANGES

* fs::open_append, fs::create_exclusive and apromise::Promise are removed from wasm-actions-core.

### Features

* add wasm-actions-node-sys ([#75](https://github.com/oakcask/wasm-actions/issues/75)) ([ec84064](https://github.com/oakcask/wasm-actions/commit/ec840644f23ffd12683a6f0d01fb2bd6b00dd3d7))
* implement File and write operation ([#80](https://github.com/oakcask/wasm-actions/issues/80)) ([6434863](https://github.com/oakcask/wasm-actions/commit/643486381e53d58fa8433adfdd65ebdfc07a9e00))
* usize implements TryFrom&lt;Integer&gt; ([#81](https://github.com/oakcask/wasm-actions/issues/81)) ([1778812](https://github.com/oakcask/wasm-actions/commit/1778812e12867e8634edf00569f6f64facc01106))


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#76](https://github.com/oakcask/wasm-actions/issues/76)) ([acccffa](https://github.com/oakcask/wasm-actions/commit/acccffad700b1fa6a820cedaab8e4477a5cbea4c))
