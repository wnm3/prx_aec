# aec-rs

[![PyPi Version](https://img.shields.io/pypi/v/pyaec?color=00aa00&logo=python)](https://pypi.org/project/pyaec/)
[![Crates](https://img.shields.io/crates/v/aec-rs?logo=rust)](https://crates.io/crates/aec-rs/)

Acoustic echo cancellation in Rust based on [speexdsp](https://github.com/xiph/speexdsp)

## Features

- Cancel echo by simple function call
- Noise supression
- Suitable for realtime
- Support Rust and Python
- Easy integration with C/C++ (or any other language) via C API
- Precompiled library and C header files available in the [releases](https://github.com/thewh1teagle/aec-rs/releases/latest)

## Install

```console
cargo add aec-rs
```

## Example

See [examples](examples)

## Build

See [building](BUILDING.md)
