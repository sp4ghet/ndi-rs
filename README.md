# ndi-rs

NewTek NDI® bindings for rust.

very WIP (as of 7/7/2021) and I have no idea what I'm doing.

Currently only supports Windows x64, but it should be possible to support other platforms eventually by linking to the respective platform SDK.

## Requirements
This crate uses [`bindgen`](https://docs.rs/bindgen/0.58.1/bindgen/) and so requires the dependencies that it has which are described [here](https://rust-lang.github.io/rust-bindgen/requirements.html)

## Building

```sh
cargo build
```


## Running Example

```sh
cargo run --example main
```


-----

NDI® is a registered trademark of NewTek, Inc.
http://ndi.tv/
