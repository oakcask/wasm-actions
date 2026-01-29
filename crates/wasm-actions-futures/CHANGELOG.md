# Changelog

## [0.3.1](https://github.com/oakcask/wasm-actions/compare/wasm-actions-futures-v0.3.0...wasm-actions-futures-v0.3.1) (2026-01-29)


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#173](https://github.com/oakcask/wasm-actions/issues/173)) ([e81ae53](https://github.com/oakcask/wasm-actions/commit/e81ae5328a0748fa3a48717288ccdd2bb59aa15c))

## [0.3.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-futures-v0.2.0...wasm-actions-futures-v0.3.0) (2026-01-04)


### Features

* implement Into&lt;JoinHandle&lt;JsValue, JsValue&gt;&gt; for Promise ([#116](https://github.com/oakcask/wasm-actions/issues/116)) ([5d0f411](https://github.com/oakcask/wasm-actions/commit/5d0f41149d2ce818117dd1a1fed4548fa4da2e44))
* provide shorthand type `UnknownPromise` ([#118](https://github.com/oakcask/wasm-actions/issues/118)) ([d831344](https://github.com/oakcask/wasm-actions/commit/d831344cf9bcd0ea2f46a11eec6470946bbd1617))


### Bug Fixes

* prevent resolution error caused by `#[wasm_action]` ([#122](https://github.com/oakcask/wasm-actions/issues/122)) ([f149fb7](https://github.com/oakcask/wasm-actions/commit/f149fb772ddb0a3b697471bf1d49f25497fe33ff))

## [0.2.0](https://github.com/oakcask/wasm-actions/compare/wasm-actions-futures-v0.1.0...wasm-actions-futures-v0.2.0) (2025-12-03)


### ⚠ BREAKING CHANGES

* reduce dependencies needed for `wasm_action` ([#105](https://github.com/oakcask/wasm-actions/issues/105))
* from_promise is now associated function of JoinHandle ([#97](https://github.com/oakcask/wasm-actions/issues/97))

### Code Refactoring

* from_promise is now associated function of JoinHandle ([#97](https://github.com/oakcask/wasm-actions/issues/97)) ([ee53204](https://github.com/oakcask/wasm-actions/commit/ee53204ca2a5c8fc1cdcba620f487c6110fdcac2))
* reduce dependencies needed for `wasm_action` ([#105](https://github.com/oakcask/wasm-actions/issues/105)) ([75481d7](https://github.com/oakcask/wasm-actions/commit/75481d756d758b0a391e9d29b58d52f1bdca3cff))

## 0.1.0 (2025-11-30)


### Features

* add wasm-actions-futures ([#77](https://github.com/oakcask/wasm-actions/issues/77)) ([a2da745](https://github.com/oakcask/wasm-actions/commit/a2da745d4a98e09710ac2d96c44011dc04fd62cb))


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([#78](https://github.com/oakcask/wasm-actions/issues/78)) ([8bbda49](https://github.com/oakcask/wasm-actions/commit/8bbda497adf32a070a81be32d2598e9768139724))
