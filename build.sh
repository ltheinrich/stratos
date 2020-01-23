#!/bin/sh

cargo clean
mkdir -p target/bin

cross build --release --target x86_64-unknown-linux-musl
x86_64-linux-gnu-strip target/x86_64-unknown-linux-musl/release/scli
x86_64-linux-gnu-strip target/x86_64-unknown-linux-musl/release/sws
cp target/x86_64-unknown-linux-musl/release/scli target/bin/x86_64-linux-scli
cp target/x86_64-unknown-linux-musl/release/sws target/bin/x86_64-linux-sws

cross build --release --target armv7-unknown-linux-musleabihf
arm-linux-gnueabihf-strip target/armv7-unknown-linux-musleabihf/release/scli
arm-linux-gnueabihf-strip target/armv7-unknown-linux-musleabihf/release/sws
cp target/armv7-unknown-linux-musleabihf/release/scli target/bin/armv7-linux-scli
cp target/armv7-unknown-linux-musleabihf/release/sws target/bin/armv7-linux-sws

cross build --release --target x86_64-pc-windows-gnu
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/scli.exe
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/sws.exe
cp target/x86_64-pc-windows-gnu/release/scli.exe target/bin/x86_64-windows-scli.exe
cp target/x86_64-pc-windows-gnu/release/sws.exe target/bin/x86_64-windows-sws.exe
