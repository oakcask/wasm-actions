# Changelog

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
