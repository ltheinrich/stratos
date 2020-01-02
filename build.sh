#!/bin/sh

mkdir -p target/bin

cargo build --release --target x86_64-unknown-linux-musl
x86_64-linux-gnu-strip target/x86_64-unknown-linux-musl/release/scli
x86_64-linux-gnu-strip target/x86_64-unknown-linux-musl/release/sws
cp target/x86_64-unknown-linux-musl/release/scli target/bin/x86_64-linux-scli
cp target/x86_64-unknown-linux-musl/release/sws target/bin/x86_64-linux-sws

cargo build --release --target armv7-unknown-linux-gnueabihf
arm-linux-gnueabihf-strip target/armv7-unknown-linux-gnueabihf/release/scli
arm-linux-gnueabihf-strip target/armv7-unknown-linux-gnueabihf/release/sws
cp target/armv7-unknown-linux-gnueabihf/release/scli target/bin/armv7-linux-scli
cp target/armv7-unknown-linux-gnueabihf/release/sws target/bin/armv7-linux-sws

cargo build --release --target aarch64-unknown-linux-gnu
aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/scli
aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/sws
cp target/aarch64-unknown-linux-gnu/release/scli target/bin/aarch64-linux-scli
cp target/aarch64-unknown-linux-gnu/release/sws target/bin/aarch64-linux-sws
