## Description

A small test of Rust/wasm based on official tutorial (https://rustwasm.github.io/book/introduction.html).

## How to build

Mostly follow https://rustwasm.github.io/book/introduction.html:

```shell
$ rustup update
$ rustup install nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
$ brew install node
$ cargo +nightly install wasm-bindgen-cli
```

To test locally:

```shell
$ npm run build-release
$ npm run serve
```
