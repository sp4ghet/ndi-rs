# ndi-rs

[![Crate at crates.io](https://img.shields.io/crates/v/ndi.svg)](https://crates.io/crates/ndi)
[![Documentation at docs.rs](https://shields.io/docsrs/ndi)](https://docs.rs/ndi/)


NewTek NDI® bindings for rust.

very WIP (as of 7/7/2021) and I have no idea what I'm doing.

Currently supports Windows x64 and Linux x64, but it should be possible to support other platforms eventually by linking to the respective platform SDK.

## Requirements
This crate uses [`bindgen`](https://docs.rs/bindgen/0.58.1/bindgen/) and so requires the dependencies that it has which are described [here](https://rust-lang.github.io/rust-bindgen/requirements.html)

## Building

```sh
#optional: generate new bindings
cargo xtask bindgen

cargo build
```


## Running Example


```sh
cd ndi-examples
cargo run --bin recv
```

```sh
cargo run --package ndi-examples --bin recv
```


-----

NDI® is a registered trademark of NewTek, Inc.
http://ndi.tv/
