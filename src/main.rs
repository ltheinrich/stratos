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
use std::fs::File;
use std::io::prelude::Read;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::{env, thread};

// Main function
fn main() {
    // init
    init_version();

    // parse arguments
    let args: Vec<String> = env::args().collect();
    let cmd = Command::from(&args, &["log", "help"]);
    if cmd.is_option("help") {
        return println!("{}", HELP);
    }

    // config options
    let mut port = "3490";
    let mut addr = "[::]";
    let mut size = 10;
    let mut threads = 2;
    let mut log = false;

    // parse config file
    let mut buf = String::new();
    if let Ok(mut file) = File::open("/etc/stratos.conf") {
        if file.read_to_string(&mut buf).is_ok() {
            conf_file(
                &mut buf,
                &mut port,
                &mut addr,
                &mut size,
                &mut threads,
                &mut log,
            );
        }
    }

    // parse cli config
    conf_cli(
        &cmd,
        &mut port,
        &mut addr,
        &mut size,
        &mut threads,
        &mut log,
    );

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

// Get cli configuration
fn conf_cli<'a>(
    cmd: &'a Command,
    port: &mut &'a str,
    addr: &mut &'a str,
    size: &mut usize,
    threads: &mut u8,
    log: &mut bool,
) {
    // parse command-line options
    if let Some(v) = cmd.get_parameter("port") {
        *port = v;
    }
    if let Some(v) = cmd.get_parameter("addr") {
        *addr = v;
    }
    if let Some(v) = cmd.get_parameter("size") {
        if let Ok(v) = v.parse() {
            *size = v;
        }
    }
    *size *= 1_048_576; // byte to mb
    if let Some(v) = cmd.get_parameter("threads") {
        // parse to u8
        if let Ok(v) = v.parse() {
            *threads = if v > 0 { v - 1 } else { v };
        }
    }
    *log = cmd.is_option("log") || *log; // each activates logging
}

// Parse file config
fn conf_file<'a>(
    buf: &'a mut String,
    port: &mut &'a str,
    addr: &mut &'a str,
    size: &mut usize,
    threads: &mut u8,
    log: &mut bool,
) {
    // parse file config
    buf.split('\n') // split lines
        .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect()) // seperate and trim key and value
        .for_each(|kv: Vec<&str>| {
            if kv.len() == 2 {
                match kv[0] {
                    "port" => *port = kv[1],
                    "addr" => *addr = kv[1],
                    "size" => {
                        // parse to usize
                        if let Ok(v) = kv[1].parse() {
                            *size = v;
                        }
                    }
                    "threads" => {
                        // parse to u8
                        if let Ok(v) = kv[1].parse() {
                            *threads = if v > 0 { v - 1 } else { v };
                        }
                    }
                    "log" => {
                        // parse to bool
                        if let Ok(v) = kv[1].parse() {
                            *log = v;
                        }
                    }
                    _ => {}
                }
            }
        });
}
