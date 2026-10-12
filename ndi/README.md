# ndi-rs

Rust bindings and low-level wrappers for sending and receiving NDI audio, video, and metadata.

Requires Rust 1.88 or later. Bundled runtimes support Windows x86_64 (MSVC), Linux x86_64 (glibc), and Intel macOS. Native Apple Silicon and other architectures are not included.

The bundled proprietary runtimes are legacy NDI 4-era binaries, not the latest SDK. Ordinary builds use checked-in bindings and do not require Clang. The macOS raw bindings include some newer declarations not exported by its bundled runtime; the high-level wrapper uses the available API subset.

`AudioData::from_buffer` is an unsafe constructor for caller-owned planar `f32` storage; follow its documented alignment, stride, and lifetime contract. Keep video buffers and metadata alive while they are used, including pending asynchronous sends. All SDK objects and frames must be dropped before optional global runtime cleanup.

Cargo arranges runtime-library lookup for `cargo run` and `cargo test`. Distributing a standalone executable requires deploying the NDI library and configuring its loader path or rpath.

See the [repository README](https://github.com/sp4ghet/ndi-rs#readme) for examples, development commands, runtime deployment, and platform limitations.

NDI runtime binaries have their own SDK license, separate from this Rust crate's license.
