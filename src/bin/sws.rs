//! Stratos web service

use http::{respond, HttpRequest};
use kern::net::Stream;
use kern::Error;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use stratos::*;

static PAGE: &[u8] = include_bytes!("../../web.html");

fn main() {
    init_version();
    let listener = TcpListener::bind("[::]:3490")
        .expect("Das TCP-Server konnte nicht am angegebenen Port starten");
    loop {
        handle(listener.accept());
    }
}

// Handle connection
fn handle(accept: io::Result<(TcpStream, SocketAddr)>) {
    thread::spawn(move || {
        if let Ok((mut stream, _)) = accept {
            if let Ok((header, rest)) = read_header(&mut stream) {
                let http_request = HttpRequest::from(&header, rest, &mut stream);
                let post_params = http_request.post();
                println!("{:?}\n{:?}", http_request, post_params);
                respond(&mut stream, PAGE, "text/html").unwrap();
            }
        }
    });
}

// Read until \r\n\r\n
fn read_header(stream: &mut TcpStream) -> Result<(String, Vec<u8>), Error> {
    let mut header = Vec::new();
    let mut rest = Vec::new();
    let mut buf = vec![0u8; 8192];
    'l: loop {
        let length = match stream.r(&mut buf) {
            Ok(length) => length,
            Err(err) => return Error::from(err),
        };
        for (i, &c) in buf.iter().enumerate() {
            if c == b'\r' {
                if buf.len() < i + 4 {
                    let mut buf_temp = vec![0u8; buf.len() - (i + 4)];
                    match stream.r(&mut buf_temp) {
                        Ok(_) => {}
                        Err(err) => return Error::from(err),
                    };
                    let buf2 = [&buf[..], &buf_temp[..]].concat();
                    if buf2[i + 1] == b'\n' && buf2[i + 2] == b'\r' && buf2[i + 3] == b'\n' {
                        header.append(&mut buf);
                        header.append(&mut buf_temp);
                        break 'l;
                    }
                } else if buf[i + 1] == b'\n' && buf[i + 2] == b'\r' && buf[i + 3] == b'\n' {
                    for &b in buf.iter().take(i + 4) {
                        header.push(b);
                    }
                    for &b in buf.iter().take(length).skip(i + 4) {
                        rest.push(b);
                    }
                    break 'l;
                } else if i + 1 == buf.len() {
                    for &b in buf.iter().take(i + 4) {
                        header.push(b);
                    }
                    for &b in buf.iter().take(length).skip(i + 4) {
                        rest.push(b);
                    }
                }
            }
        }
    }
    Ok((
        match String::from_utf8(header) {
            Ok(header) => header,
            Err(err) => return Error::from(err),
        },
        rest,
    ))
}
