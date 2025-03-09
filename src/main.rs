//! Stratos main

mod analyze;
mod common;
mod handler;
mod parse;

use common::*;
use handler::handle;
use kern::http::server::{HttpServerBuilder, HttpSettings, certificate_config};
use kern::{CliBuilder, Config, meta::init_version};
use parse::Log;
use rustls::ServerConfig;
use std::env;
use std::sync::Arc;

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

    // HTTP settings
    let http_settings = HttpSettings::new().max_body_size(size).threads_num(threads);

    // listen
    let listen_addr = format!("{addr}:{port}");
    let server = HttpServerBuilder::new()
        .addr(listen_addr)
        .settings(http_settings)
        .tls_on(tls_config)
        .handler(handle)
        .build()
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

fn tls_config() -> Arc<ServerConfig> {
    Arc::new(
        certificate_config(
            include_bytes!("../data/cert.pem"),
            include_bytes!("../data/key.pem"),
        )
        .unwrap(),
    )
}
