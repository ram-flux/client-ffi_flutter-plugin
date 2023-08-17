# Download targets for IOS ( 64 bit targets (real device & simulator) )
rustup target add aarch64-apple-ios x86_64-apple-ios
# rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

 
# Install cargo-lipo to generate the iOS universal library
cargo install cargo-lipo
cargo lipo --release 

cp ../target/universal/release/libclient_ffi.a  ../client-ffi_flutter-plugin/client_ffi_demo/ios