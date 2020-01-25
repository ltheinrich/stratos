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
use kern::cli::Command;
use parse::Log;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::thread;

// Main function
fn main() {
    // init
    init_version();
    let args: Vec<String> = env::args().collect();
    let cmd = Command::from(&args, &["log", "help"]);
    if cmd.is_option("help") {
        return println!("{}", HELP);
    }

    // parse command line
    let port = cmd.get_parameter("port").unwrap_or(&"3490");
    let addr = cmd.get_parameter("addr").unwrap_or(&"[::]");
    let size: usize = cmd
        .get_parameter("size")
        .unwrap_or(&"10")
        .parse()
        .expect("Das angegebene Log-Größen-Limit ist kein Integer");
    let size = size * 1_048_576;
    let threads: u8 = cmd
        .get_parameter("threads")
        .unwrap_or(&"2")
        .parse()
        .expect("Die angegebene Threads-Anzahl ist kein Integer");
    let threads = threads - 1;
    let log = cmd.is_option("log");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", addr, port))
        .expect("Das TCP-Server konnte nicht an der angegebenen Adresse bzw. Port starten");
    let listener = Arc::new(RwLock::new(listener));
    println!("Der Server läuft unter  {}:{}", addr, port);
    (0..threads).for_each(|_| {
        let listener = listener.clone();
        thread::spawn(move || accept_connections(listener, log, size));
    });
    thread::spawn(move || accept_connections(listener, log, size))
        .join()
        .unwrap();
}
