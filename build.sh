#!/bin/sh

cargo update
cargo clean
mkdir -p target/bin

cross build --release --target x86_64-unknown-linux-musl
cargo deb --no-build --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/stratos target/bin/x86_64-linux-stratos

cross build --release --target armv7-unknown-linux-musleabihf
cargo deb --no-build --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/stratos target/bin/armv7-linux-stratos

cross build --release --target x86_64-pc-windows-gnu
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/stratos.exe
cp target/x86_64-pc-windows-gnu/release/stratos.exe target/bin/x86_64-windows-stratos.exe
