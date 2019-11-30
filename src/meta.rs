//! Stratos metadata

use kern::meta;

const CARGO_TOML: &str = include_str!("../Cargo.toml");

pub fn version() {
    println!(
        "Stratos {} (c) 2019 Lennart Heinrich\n",
        meta::version(CARGO_TOML)
    );
}
