# ndi-rs

[![Crate at crates.io](https://img.shields.io/crates/v/ndi.svg)](https://crates.io/crates/ndi)
[![Documentation at docs.rs](https://shields.io/docsrs/ndi)](https://docs.rs/ndi/)
[![CI](https://github.com/sp4ghet/ndi-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/sp4ghet/ndi-rs/actions/workflows/ci.yml)

Rust bindings and low-level wrappers for sending and receiving NDI audio, video, and metadata.

## Requirements and platform support

- Rust **1.88 or later**. CI also tests current stable Rust.
- Windows x86_64 (MSVC), Linux x86_64 (glibc), or macOS x86_64 (Intel).
- A CPU supported by the bundled NDI runtime. Native Apple Silicon, ARM Linux, Windows GNU, and 32-bit targets are not supported by the bundled bindings and runtimes.

The repository includes legacy **NDI 4-era runtime binaries**. This maintenance refresh updates Rust dependencies and tooling; it does not upgrade the proprietary NDI SDK. The macOS contribution includes newer SDK headers, but its bundled runtime is still v4 and Intel-only. The high-level wrappers use functions available in that runtime; the hidden raw bindings can expose newer functions that it does not export. Do not assume current NDI SDK features are available.

Ordinary builds use checked-in Rust bindings and do **not** require Clang or an SDK download. Regenerating bindings requires [bindgen's Clang/libclang dependencies](https://rust-lang.github.io/rust-bindgen/requirements.html).

## Build and test

Run from the repository root:

```sh
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
```

Tests include ABI layout checks, audio-buffer contracts, runtime initialization, sender transfer between threads, and receive timeouts. CI runs on Windows, Linux, and Intel macOS, plus the minimum supported Rust version on Linux. These tests do not require another NDI source on the network.

The workspace uses one root `Cargo.lock`. Examples and the binding generator are development packages and are not published.

## Examples

```sh
cargo run -p ndi-examples --bin recv
cargo run -p ndi-examples --bin send
```

The `recv`, `save_recv`, `multithread`, and `data_lifetime` examples require an available NDI source. `send` creates its own sender/receiver, waits up to 15 seconds for a video frame, and writes `send.png` in the working directory. Local networking/firewall settings must allow the NDI connection.

The `AudioData::from_buffer` constructor describes caller-owned planar `f32` samples without copying them. It is **unsafe**: the caller must provide valid dimensions, alignment, channel stride, and storage that outlives the frame and all uses of it. See its API safety documentation. Video buffers and metadata likewise need to remain alive while in use; asynchronous video sends keep using the buffer until a synchronizing send or sender destruction.

Drop all NDI objects and captured frames before calling the optional unsafe `ndi::cleanup()`. The examples let their objects drop on exit without explicitly destroying the global runtime early.

## Runtime libraries

The build script copies the platform library into Cargo's `OUT_DIR`, including the Unix linker and runtime filenames. Cargo configures the loader path for `cargo run` and `cargo test`.

When distributing or running an executable directly, arrange runtime loading yourself:

- Windows: put `Processing.NDI.Lib.x64.dll` beside the executable or on `PATH`.
- Linux: install `libndi.so.4` on the loader path, set `LD_LIBRARY_PATH`, or package it with an appropriate rpath.
- macOS: package `libndi.4.dylib` and configure an rpath that resolves its `@rpath/libndi.4.dylib` install name, for example `@executable_path` for a library beside the executable.

The NDI runtime binaries are subject to the SDK's own license, separate from this Rust crate's license.

## Regenerate bindings

After installing Clang/libclang, run this on the platform whose bindings you want to regenerate:

```sh
cargo xtask bindgen
cargo fmt --all
cargo test --workspace --locked
```

The generator uses the bundled platform headers and emits Rust 2021-compatible bindings targeting the workspace's minimum Rust version. Check changes before committing; generation on a different SDK or architecture can change the ABI. The existing Windows/Linux layout tests use `offset_of!` to avoid the null-pointer dereferences emitted by old bindgen versions.

## Dependency maintenance

Dependabot checks Cargo dependencies and GitHub Actions monthly. The examples enable only the PNG feature of `image`, which is the format they write. `bindgen` and `xflags` are used only by `xtask`.

To refresh and audit dependencies:

```sh
cargo update
cargo install cargo-audit --locked
cargo audit --deny warnings
```

Run the build, test, formatting, and Clippy checks above before merging. RustSec auditing covers Rust dependencies, not the bundled proprietary NDI binaries.

---

NDI is a registered trademark of its respective owner. See [NDI](https://ndi.video/).
