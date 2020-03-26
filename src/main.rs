//! Stratos main

extern crate kern;
extern crate plotlib;

mod analyze;
mod common;
mod handler;
mod http;
mod parse;

use common::*;
use handler::*;
use kern::{init_version, Command, Config};
use parse::Log;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::{env, thread};

// Main function
fn main() {
    // init
    println!(
        "Stratos {} (c) 2019 Lennart Heinrich\n",
        init_version(CARGO_TOML)
    );

    // parse arguments
    let args: Vec<String> = env::args().collect();
    let cmd = Command::from(&args, &["log", "help"]);
    if cmd.option("help") {
        return println!("{}", HELP);
    }
    // load file config
    let mut conf_buf = String::new();
    let config =
        Config::read("/etc/stratos.conf", &mut conf_buf).unwrap_or_else(|_| Config::from(""));

    // configuration
    let port = cmd.param("port", config.value("port", "3490"));
    let addr = cmd.param("addr", config.value("addr", "[::]"));
    // threads: default value must be -1 the actual default value
    let threads = cmd.parameter("threads", config.get("threads", 1));
    let size = cmd.parameter("size", config.get("size", 10)) * 1_048_576;
    let log = cmd.parameter("log", config.get("log", false));

    // start server
    let listener = TcpListener::bind(format!("{}:{}", addr, port))
        .expect("Das TCP-Server konnte nicht an der angegebenen Adresse bzw. Port starten");
    let listener = Arc::new(RwLock::new(listener));

    // start threads
    (0..threads).for_each(|_| {
        let listener = listener.clone();
        thread::spawn(move || accept_connections(listener, log, size));
    });

    // print info message
    if addr == "[::]" {
        // default message
        println!("Öffne Stratos im Browser: http://localhost:{}", port);
    } else {
        // more technical ;)
        println!("Der Server läuft unter {}:{}", addr, port);
    }

    // final thread
    thread::spawn(move || accept_connections(listener, log, size))
        .join()
        .unwrap();
}
