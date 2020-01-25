#!/bin/sh

cargo clean
mkdir -p target/bin

cross build --release --target x86_64-unknown-linux-musl
x86_64-linux-gnu-strip target/x86_64-unknown-linux-musl/release/stratos
cp target/x86_64-unknown-linux-musl/release/stratos target/bin/x86_64-linux-stratos

cross build --release --target armv7-unknown-linux-musleabihf
arm-linux-gnueabihf-strip target/armv7-unknown-linux-musleabihf/release/stratos
cp target/armv7-unknown-linux-musleabihf/release/stratos target/bin/armv7-linux-stratos

cross build --release --target x86_64-pc-windows-gnu
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/stratos.exe
cp target/x86_64-pc-windows-gnu/release/stratos.exe target/bin/x86_64-windows-stratos.exe
