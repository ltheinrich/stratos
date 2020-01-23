//! Stratos web service

use http::{respond, HttpMethod, HttpRequest};
use kern::cli::Command;
use kern::net::Stream;
use kern::Error;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use stratos::*;

const PAGE: &[u8] = include_bytes!("../../web/index.html");
const HEAD: &str = include_str!("../../web/head.html");
const END: &str = "</body></html>";
const BOOTSTRAP: &[u8] = include_bytes!("../../web/bootstrap.min.css");
const BOOTSTRAP_MAP: &[u8] = include_bytes!("../../web/bootstrap.min.css.map");
const FAVICON_ICO: &[u8] = include_bytes!("../../web/favicon.ico");
const FAVICON_PNG: &[u8] = include_bytes!("../../web/favicon.png");
const APPLE_TOUCH_ICON: &[u8] = include_bytes!("../../web/apple-touch-icon.png");
const HELP: &str = "Benutzung: sws [OPTIONEN]\nString S, Integer I, Boolean B\n\nOptionen:
  --port    I       Port (3490)
  --addr    S       IP-Adresse ([::])
  --size    I       Maximale Log-Größe in MB (10)
  --log             Logging der Anfragen aktivieren";

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
    let log = cmd.is_option("log");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", addr, port))
        .expect("Das TCP-Server konnte nicht an der angegebenen Adresse bzw. Port starten");
    let listener = Arc::new(RwLock::new(listener));
    println!("Der Server läuft unter  {}:{}", addr, port);
    (0..10).for_each(|_| {
        let listener = listener.clone();
        thread::spawn(move || accept_connections(listener, log, size));
    });
    thread::spawn(move || accept_connections(listener, log, size))
        .join()
        .unwrap();
}

// Accept connections
fn accept_connections(listener: Arc<RwLock<TcpListener>>, log: bool, size: usize) {
    loop {
        if let Ok((stream, addr)) = listener.read().unwrap().accept() {
            thread::spawn(move || {
                if log {
                    println!("Log: Anfrage von {}", addr.ip());
                }
                handle_connection(stream, size);
            });
        }
    }
}

// Handle connection
fn handle_connection(mut stream: TcpStream, max_content: usize) {
    if let Ok((header, rest)) = read_header(&mut stream) {
        let http_request = match HttpRequest::from(&header, rest, &mut stream, max_content) {
            Some(http_request) => http_request,
            None => {
                return respond(
                    &mut stream,
                    format!(
                        "{}<div class=\"alert alert-danger\" role=\"alert\">Die HTTP-Anfrage konnte nicht gelesen werden</div>{}",
                        HEAD, END
                    )
                    .as_bytes(),
                    "text/html",
                    None,
                )
                .unwrap()
            }
        };
        if http_request.method() == &HttpMethod::POST {
            let post_params = match http_request.post() {
                Some(post_params) => post_params,
                None => {
                    return respond(
                        &mut stream,
                        format!(
                            "{}<div class=\"alert alert-danger\" role=\"alert\">Die POST-Anfrage konnte nicht gelesen werden</div><br><p>Möglicherweise hast du keine Log-Datei ausgewählt oder dein Browser wird nicht unterstützt (nutze in diesem Fall Firefox)</p>{}",
                            HEAD, END
                        )
                        .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            };
            let file = match post_params.get("file") {
                Some(file) => file,
                None => {
                    return respond(
                        &mut stream,
                        format!("{}<div class=\"alert alert-danger\" role=\"alert\">Bitte suche eine Log-Datei aus</div>{}", HEAD, END)
                            .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            };
            let x_axis = match post_params.get("x") {
                Some(x_axis) => x_axis,
                None => {
                    return respond(
                        &mut stream,
                        format!(
                            "{}<div class=\"alert alert-danger\" role=\"alert\">Die Angabe der x-Achse ist erforderlich</div>{}",
                            HEAD, END
                        )
                        .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            };
            let y_axis = match post_params.get("y") {
                Some(y_axis) => y_axis,
                None => {
                    return respond(
                        &mut stream,
                        format!(
                            "{}<div class=\"alert alert-danger\" role=\"alert\">Die Angabe der y-Achse ist erforderlich</div>{}",
                            HEAD, END
                        )
                        .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            };
            let analysis = match draw(
                &file,
                Parameters::from(
                    x_axis,
                    y_axis,
                    none_empty(post_params.get("xn")),
                    none_empty(post_params.get("yn")),
                    none_empty(post_params.get("xmin")),
                    none_empty(post_params.get("xmax")),
                    none_empty(post_params.get("ymin")),
                    none_empty(post_params.get("ymax")),
                    none_empty(post_params.get("s")),
                    none_empty(post_params.get("c")),
                    none_empty(post_params.get("cf")),
                    none_empty(post_params.get("h")),
                ),
            ) {
                Ok(analysis) => analysis,
                Err(err) => {
                    return respond(
                        &mut stream,
                        format!(
                            "{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                            HEAD,
                            err.to_string(),
                            END
                        )
                        .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            };
            respond(
                &mut stream,
                analysis.as_bytes(),
                "image/svg",
                Some("analysis.svg"),
            )
            .unwrap();
        } else {
            match &http_request.url()[1..] {
                "bootstrap.min.css" => respond(&mut stream, BOOTSTRAP, "text/css", None).unwrap(),
                "bootstrap.min.css.map" => {
                    respond(&mut stream, BOOTSTRAP_MAP, "text/css", None).unwrap()
                }
                "favicon.ico" => respond(&mut stream, FAVICON_ICO, "image/x-icon", None).unwrap(),
                "favicon.png" => respond(&mut stream, FAVICON_PNG, "image/png", None).unwrap(),
                "apple-touch-icon.png" => {
                    respond(&mut stream, APPLE_TOUCH_ICON, "image/png", None).unwrap()
                }
                _ => respond(&mut stream, PAGE, "text/html", None).unwrap(),
            }
        }
    }
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

// None instead of ""
fn none_empty<'a, 'b>(opt: Option<&'a &'b str>) -> Option<&'a &'b str> {
    if let Some(value) = opt {
        if value == &"" {
            return None;
        }
    }
    opt
}
