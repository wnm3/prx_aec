# Building

Ensure you have installed rust:
brew install rust

Ensure you have installed cmake:
brew install cmake


```console
git clone https://github.com/wnm3/prx_aec --recursive
cd prx_aec
git submodule init
git submodule update
cargo build
```

Useful website for comparesion:

https://fjiang9.github.io/NKF-AEC/

## Publish crates (skip this step)

```console
cd crates/aec-rs-sys
cargo publish
cd ../../
cargo publish
```

## Build for Android

You must install NDK from Android Studio settings.

```console
rustup target add aarch64-linux-android
cargo install cargo-ndk
export NDK_HOME="$HOME/Library/Android/sdk/ndk/$(ls -1 $HOME/Library/Android/sdk/ndk | sort | tail -n 1)"
export CMAKE_TOOLCHAIN_FILE="$NDK_HOME/build/cmake/android.toolchain.cmake"
cargo ndk -t arm64-v8a build --release -p libaec
```

## Build for IOS

Install Xcode command line tools

```console
xcode-select --install
```

Install toolchain

```console
# IOS
rustup target add aarch64-apple-ios
# Intel chip emulator
rustup target add x86_64-apple-ios
# Apple chip emulator
rustup target add aarch64-apple-ios-sim
```

Build

```console
cargo build --release --target aarch64-apple-ios
```

## Build for wasm

Use [wasm-pack](https://rustwasm.github.io/docs/wasm-pack) with [emscripten.org](https://emscripten.org)

```console
brew install emscripten
rustup target add wasm32-unknown-emscripten
cargo build --release --target wasm32-unknown-emscripten
CC=emcc AR=emar wasm-pack build
```

## Build pyaec (Python)

Use [uv](https://astral.sh/blog/uv)

Note: if building for MacOS on intel substitute:  
WHEEL_TAG="py3-none-macosx_10_12_x86_64"  
below...  

```console
cd prx_pyaec
cargo build -p libaec --release
cp -rf ../target/release/libaec.dylib src/pyaec/
WHEEL_TAG="py3-none-macosx_11_0_arm64" uv build
```

Publish

```console
export UV_PUBLISH_TOKEN="..."
uv publish
```

Installing once published

```console
uv run --with pyaec --refresh-package pyaec --no-project -- python -c "import pyaec"
```
