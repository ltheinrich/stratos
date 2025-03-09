//! Web handler

use crate::common::*;
use crate::parse::Log;
use kern::http::server::{HttpMethod, HttpRequest, ResponseData, redirect, respond};
use kern::{Fail, Result};
use std::collections::HashMap;

// Handle HTTP request
pub fn handle(req: HttpRequest) -> Result<Vec<u8>> {
    // match URL
    Ok(match &req.url()[1..] {
        "favicon.ico" => respond(FAVICON_ICO, "image/x-icon", None),
        "favicon.png" => respond(FAVICON_PNG, "image/png", None),
        "apple-touch-icon.png" => respond(APPLE_TOUCH_ICON, "image/png", None),
        "bootstrap.min.css" => respond(BOOTSTRAP, "text/css", None),
        "style.css" => respond(STYLE, "text/css", None),
        _ => {
            // check if POST
            if req.method() == &HttpMethod::Post {
                // process POST request
                println!("post");
                process_request(&req)
            } else if req.get().contains_key("options") {
                // redirect GET with ?options to /
                redirect("/")
            } else {
                // serve index page
                respond(
                    format!("{}{}{}", HEAD, INDEX.replace("%LOG_FILE%", ""), footer()).as_bytes(),
                    "text/html",
                    None,
                )
            }
        }
    })
}

// Process HTTP POST request
fn process_request(http_request: &HttpRequest) -> Vec<u8> {
    // parse post map as utf-8
    let post = http_request.post_utf8();

    // raw log file
    let file = match post.get("file") {
        Some(file) => file,
        None => {
            // error
            return respond(
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">Bitte suche eine Log-Datei aus</div>{}",
                    HEAD,
                    BACK,
                    footer()
                ),
                "text/html",
                None,
            );
        }
    };
    // check if options page
    if http_request.get().contains_key("options") {
        // serve options page
        println!("options");
        handle_options(&post, file)
    } else {
        // serve index page
        handle_index(file)
    }
}

// Handle options page
fn handle_options(post_params: &HashMap<String, String>, file: &str) -> Vec<u8> {
    // get x-axis and y-axis
    println!("xy");
    let (x_axis, y_axis) = match get_xy_names(post_params) {
        Ok((x_axis, y_axis)) => (x_axis, y_axis),
        Err(err) => {
            return respond(
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                    HEAD,
                    BACK,
                    err,
                    footer()
                ),
                "text/html",
                None,
            );
        }
    };

    // draw analysis
    println!("draw");
    let analysis = match draw(
        file,
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
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                    HEAD,
                    BACK,
                    err,
                    footer()
                )
                .as_bytes(),
                "text/html",
                None,
            );
        }
    };

    // serve analysis image
    println!("headers");
    let mut headers = HashMap::new();
    headers.insert(
        "content-disposition",
        "attachment; filename=\"analysis.svg\"",
    );
    let mut resp_data = ResponseData::new();
    resp_data.headers = headers;
    respond(analysis.as_bytes(), "image/svg", Some(resp_data))
}

// Get x-axis name and y-axis name
fn get_xy_names(post_params: &HashMap<String, String>) -> Result<(&str, &str)> {
    // x-/y-axis
    let x_axis = post_params
        .get("x")
        .ok_or_else(|| Fail::new("Die Angabe der x-Achse ist erforderlich"))?;
    let y_axis = post_params
        .get("y")
        .ok_or_else(|| Fail::new("Die Angabe der y-Achse ist erforderlich"))?;

    // return
    Ok((x_axis, y_axis))
}

// Handle index page
fn handle_index(file: &str) -> Vec<u8> {
    // parse log file
    println!("log");
    let log = match Log::from(file) {
        Ok(log) => log,
        Err(err) => {
            // error
            return respond(
                format!(
                    "{}{}<div class=\"alert alert-danger\" role=\"alert\">{}</div>{}",
                    HEAD,
                    BACK,
                    err,
                    footer()
                ),
                "text/html",
                None,
            );
        }
    };

    // serve options site
    respond(
        format!(
            "{}{}{}{}",
            HEAD,
            BACK,
            OPTIONS
                .replace("%LOG_FILE%", file)
                .replace("%HEADER_NAMES%", &header_options(log.header())),
            footer()
        )
        .as_bytes(),
        "text/html",
        None,
    )
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
