#!/bin/bash
# cargo install --force cbindgen


# cbindgen --config cbindgen.toml --crate client-ffi --output ./client-ffi.h
# cp ./client-ffi.h ./client_ffi_plugin/lib

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
cargo ndk -t armeabi-v7a -t arm64-v8a -o ../libs/android/jniLibs build --release

cp -r ../libs/android/jniLibs  ../client_ffi_demo/android/app/src/main


# flutter_rust_bridge_codegen \
# --rust-input ../dart_rust_logger/src/api.rs \
# --dart-output ./dart_rust_logger_plugin/lib/bridge_generated.dart \
# --dart-decl-output ./dart_rust_logger_plugin/lib/bridge_definitions.dart \

# cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 -o ./client_ffi_demo/android/app/src/main/jniLibs build --release
