//! Stratos metadata

use kern::meta;

const CARGO_TOML: &str = include_str!("../Cargo.toml");
static mut VERSION: &str = "";

pub fn version() -> &'static str {
    unsafe { VERSION }
}

pub fn init_version() {
    unsafe {
        VERSION = meta::version(CARGO_TOML);
        println!("Stratos {} (c) 2019 Lennart Heinrich\n", VERSION);
    }
}
