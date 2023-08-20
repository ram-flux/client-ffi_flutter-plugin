# Download targets for IOS ( 64 bit targets (real device & simulator) )
rustup target add aarch64-apple-ios x86_64-apple-ios
# rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

 
# Install cargo-lipo to generate the iOS universal library
# cargo install cargo-lipo
cd client-ffi
cargo lipo --release 

cp -r ../../target/universal/release/libclient_ffi.a   ../libs/ios/libclient_ffi.a

cp ../libs/ios/libclient_ffi.a  ../client_ffi_demo/ios
