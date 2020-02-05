//! Web handler

use crate::common::*;
use crate::http::{read_header, redirect, respond, HttpMethod, HttpRequest};
use crate::parse::Log;
use kern::Error;
use std::collections::BTreeMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

// Accept connections
pub fn accept_connections(listener: Arc<RwLock<TcpListener>>, log: bool, size: usize) {
    loop {
        // accept connection
        if let Ok((stream, addr)) = listener.read().unwrap().accept() {
            // spawn new thread
            thread::spawn(move || {
                // print log if enabled
                if log {
                    println!("Log: Anfrage von {}", addr.ip());
                }

                // handle connection
                handle_connection(stream, size);
            });
        }
    }
}

// Handle connection
fn handle_connection(mut stream: TcpStream, max_content: usize) {
    // read header
    if let Ok((header, rest)) = read_header(&mut stream) {
        // parse HTTP request
        let http_request = match HttpRequest::from(&header, rest, &mut stream, max_content) {
            Some(http_request) => http_request,
            None => {
                // error
                return respond(
                    &mut stream,
                    format!(
                        "{}{}<div class=\"alert alert-danger\" role=\"alert\">Die HTTP-Anfrage konnte nicht gelesen werden</div>{}",
                        HEAD, BACK, footer()
                    )
                    .as_bytes(),
                    "text/html",
                    None,
                )
                .unwrap();
            }
        };

        // match URL
        match &http_request.url()[1..] {
            "favicon.ico" => respond(&mut stream, FAVICON_ICO, "image/x-icon", None).unwrap(),
            "favicon.png" => respond(&mut stream, FAVICON_PNG, "image/png", None).unwrap(),
            "apple-touch-icon.png" => {
                respond(&mut stream, APPLE_TOUCH_ICON, "image/png", None).unwrap()
            }
            "bootstrap.min.css" => respond(&mut stream, BOOTSTRAP, "text/css", None).unwrap(),
            "style.css" => respond(&mut stream, STYLE, "text/css", None).unwrap(),
            _ => {
                // check if POST
                if http_request.method() == &HttpMethod::POST {
                    // process POST request
                    process_request(&http_request, &mut stream)
                } else if http_request.get().contains_key("options") {
                    // redirect GET with ?options to /
                    redirect(&mut stream, "/").unwrap()
                } else {
                    // serve index page
                    respond(
                        &mut stream,
                        format!("{}{}{}", HEAD, INDEX.replace("%LOG_FILE%", ""), footer())
                            .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap()
                }
            }
        }
    }
}

// Process HTTP POST request
fn process_request(http_request: &HttpRequest, stream: &mut TcpStream) {
    // parse post parameters
    let post_params = match http_request.post() {
        Some(post_params) => post_params,
        None => {
            // error
            return respond(
                        stream,
                        format!(
                            "{}{}<div class=\"alert alert-danger\" role=\"alert\">Die POST-Anfrage konnte nicht gelesen werden</div><small class=\"form-text text-muted\">Möglicherweise hast du keine Log-Datei ausgewählt oder dein Browser wird nicht unterstützt (nutze in diesem Fall Firefox)</small>{}",
                            HEAD, BACK, footer()
                        )
                        .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap();
        }
    };

    // raw log file
    let file = match post_params.get("file") {
        Some(file) => file,
        None => {
            // error
            return respond(
                        stream,
                        format!("{}{}<div class=\"alert alert-danger\" role=\"alert\">Bitte suche eine Log-Datei aus</div>{}", HEAD, BACK, footer())
                            .as_bytes(),
                        "text/html",
                        None,
                    )
                    .unwrap();
        }
    };

    // check if options page
    if http_request.get().contains_key("options") {
        // serve options page
        handle_options(stream, &post_params, file);
    } else {
        // serve index page
        handle_index(stream, file);
    }
}

// Handle options page
fn handle_options(stream: &mut TcpStream, post_params: &BTreeMap<&str, &str>, file: &str) {
    // get x-axis and y-axis
    let (x_axis, y_axis) = match get_xy_names(stream, post_params) {
        Ok((x_axis, y_axis)) => (x_axis, y_axis),
        Err(_) => return, // already handled :)
    };

    // draw analysis
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
            // error
            return respond(
                stream,
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                    HEAD,
                    BACK,
                    err.to_string(),
                    footer()
                )
                .as_bytes(),
                "text/html",
                None,
            )
            .unwrap();
        }
    };

    // serve analysis image
    respond(
        stream,
        analysis.as_bytes(),
        "image/svg",
        Some("analysis.svg"),
    )
    .unwrap();
}

// Get x-axis name and y-axis name
fn get_xy_names<'a>(
    stream: &mut TcpStream,
    post_params: &BTreeMap<&'a str, &'a str>,
) -> Result<(&'a str, &'a str), Error> {
    // x-axis
    let x_axis = match post_params.get("x") {
        Some(x_axis) => x_axis,
        None => {
            // error
            respond(
               stream,
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">Die Angabe der x-Achse ist erforderlich</div>{}",
                    HEAD, BACK, footer()
                )
                .as_bytes(),
                "text/html",
                None,
            )
            .unwrap();
            return Error::from("x-Achse wurde nicht angegeben");
        }
    };

    // y-axis
    let y_axis = match post_params.get("y") {
        Some(y_axis) => y_axis,
        None => {
            // error
            respond(
              stream,
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">Die Angabe der y-Achse ist erforderlich</div>{}",
                    HEAD, BACK, footer()
                )
                .as_bytes(),
                "text/html",
                None,
            )
            .unwrap();
            return Error::from("y-Achse wurde nicht angegeben");
        }
    };

    Ok((x_axis, y_axis))
}

// Handle index page
fn handle_index(stream: &mut TcpStream, file: &str) {
    // parse log file
    let log = match Log::from(file) {
        Ok(log) => log,
        Err(err) => {
            // error
            return respond(
                stream,
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                    HEAD,
                    BACK,
                    err.to_string(),
                    footer()
                )
                .as_bytes(),
                "text/html",
                None,
            )
            .unwrap();
        }
    };

    // serve options site
    respond(
        stream,
        format!(
            "{}{}{}{}",
            HEAD,
            BACK,
            OPTIONS
                .replace("LOG_FILE", file)
                .replace("%HEADER_NAMES%", &header_options(log.header())),
            footer()
        )
        .as_bytes(),
        "text/html",
        None,
    )
    .unwrap()
}

// Header names as HTML options
fn header_options(header: &[&str]) -> String {
    let mut options = String::new();
    for h in header {
        options.push_str("<option value=\"");
        options.push_str(h);
        options.push_str("\">");
        options.push_str(h);
        options.push_str("</option>");
    }
    options
}
