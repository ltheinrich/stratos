//! Stratos main

mod analyze;
mod common;
mod handler;
mod parse;

use common::*;
use handler::handle;
use kern::http::server::{certificate_config, HttpServerBuilder, HttpSettings};
use kern::{meta::init_version, CliBuilder, Config};
use parse::Log;
use std::env;

// Main function
fn main() {
    // init
    println!(
        "Stratos {} (c) 2019 Lennart Heinrich\n",
        init_version(CARGO_TOML)
    );

    // parse arguments
    let args: Vec<String> = env::args().collect();
    let cmd = CliBuilder::new().options(&["log", "help"]).build(&args);
    if cmd.option("help") {
        return println!("{HELP}");
    }

    // load file config
    let mut conf_buf = String::new();
    let config =
        Config::read("/etc/stratos.conf", &mut conf_buf).unwrap_or_else(|_| Config::from(""));

    // configuration
    let port = cmd.param("port", config.value("port", "4491"));
    let addr = cmd.param("addr", config.value("addr", "[::]"));
    let threads = cmd.parameter("threads", config.get("threads", 2));
    let size = cmd.parameter("size", config.get("size", 10)) * 1_048_576;

    // load tls config
    let tls_config = certificate_config(
        include_bytes!("../data/cert.pem"),
        include_bytes!("../data/key.pem"),
    )
    .unwrap();

    // HTTP settings
    let mut http_settings = HttpSettings::new();
    http_settings.max_body_size = size;

    // listen
    let listen_addr = format!("{addr}:{port}");
    let server = HttpServerBuilder::new()
        .addr(listen_addr)
        .threads(threads)
        .settings(http_settings)
        .tls_on(tls_config)
        .handler(handle)
        .build(Default::default())
        .expect("Der TCP-Server konnte nicht an der angegebenen Adresse bzw. Port starten");

    // print info message
    if addr == "[::]" {
        // default message
        println!("Öffne Stratos im Browser: https://localhost:{port}");
    } else {
        // more technical ;)
        println!("Der Server läuft unter {addr}:{port}");
    }

    server.block().unwrap();
}
