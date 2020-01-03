//! HTTP parsing

use crate::version;
use kern::net::Stream;
use std::collections::BTreeMap;
use std::io;
use std::net::TcpStream;

/// HTTP request method (GET or POST)
#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

/// HTTP request structure
#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    url: &'a str,
    headers: BTreeMap<&'a str, &'a str>,
    get: BTreeMap<&'a str, &'a str>,
    body: String,
}

// HTTP request implementation
impl<'a> HttpRequest<'a> {
    pub fn from(
        raw_header: &'a str,
        mut raw_body: Vec<u8>,
        stream: &mut TcpStream,
        max_content: usize,
    ) -> Self {
        let mut header = raw_header.lines();
        // parse method and url
        let mut reqln = header.next().unwrap().split(' ');
        let method = if reqln.next().unwrap() == "POST" {
            HttpMethod::POST
        } else {
            HttpMethod::GET
        };
        let mut get_raw = "";
        let url = if let Some(full_url) = reqln.next() {
            let mut split_url = full_url.splitn(2, '?');
            let url = split_url.next().unwrap();
            if let Some(params) = split_url.next() {
                get_raw = params;
            }
            url
        } else {
            "/"
        };
        // parse headers
        let mut headers = BTreeMap::new();
        header.for_each(|hl| {
            let mut hls = hl.splitn(2, ':');
            let key = hls.next().unwrap().trim();
            if let Some(value) = hls.next() {
                headers.insert(key, value.trim());
            }
        });

        // read body
        let mut body = String::new();
        stream
            .set_read_timeout(Some(std::time::Duration::from_millis(2000)))
            .unwrap();
        let buf_len = if let Some(buf_len) = headers.get("Content-Length") {
            Some(buf_len)
        } else {
            headers.get("content-length")
        };
        if let Some(buf_len) = buf_len {
            let con_len = buf_len.parse::<usize>().unwrap();
            if con_len > max_content {
                body = String::from("Maximale Log-Größe überschritten");
            } else {
                while raw_body.len() < con_len {
                    let mut rest_body = vec![0u8; 65536];
                    let length = stream.r(&mut rest_body).unwrap();
                    rest_body.truncate(length);
                    raw_body.append(&mut rest_body);
                }
                body = String::from_utf8(raw_body).unwrap();
            }
        }

        // parse GET parameters and return
        let get = parse_parameters(get_raw);
        Self {
            method,
            url,
            headers,
            get,
            body,
        }
    }

    /// Parse POST parameters
    pub fn post(&self) -> BTreeMap<&str, &str> {
        if self.method == HttpMethod::POST {
            parse_upload(&self.body)
        } else {
            BTreeMap::new()
        }
    }

    /// Get HTTP request method
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Get URL
    pub fn url(&self) -> &str {
        self.url
    }

    /// Get headers map
    pub fn headers(&self) -> &BTreeMap<&str, &str> {
        &self.headers
    }

    /// Get GET parameters
    pub fn get(&self) -> &BTreeMap<&str, &str> {
        &self.get
    }
}

// Parse POST file upload with parameters to map
fn parse_upload(body: &str) -> BTreeMap<&str, &str> {
    let mut params = BTreeMap::new();
    body.split("\r\n---").for_each(|content| {
        let mut lines = content.splitn(4, "\r\n").skip(1);
        let mut name = "";
        lines
            .next()
            .unwrap()
            .split(';')
            .map(|line| line.trim())
            .for_each(|line| {
                if line.starts_with("name=") {
                    name = &line[6..(line.len() - 1)];
                }
            });
        if let Some(value) = lines.next() {
            if value == "" {
                params.insert(name, lines.next().unwrap());
            } else {
                let mut a = lines.next().unwrap().splitn(2, "\r\n");
                params.insert(name, a.nth(1).unwrap());
            }
        }
    });
    params
}

// Parse GET parameters to map
fn parse_parameters(raw: &str) -> BTreeMap<&str, &str> {
    let mut params = BTreeMap::new();
    raw.split('&').for_each(|p| {
        let mut ps = p.splitn(2, '=');
        params.insert(
            ps.next().unwrap().trim(),
            if let Some(value) = ps.next() {
                value.trim()
            } else {
                ""
            },
        );
    });
    params
}

/// HTTP responder
pub fn respond(
    stream: &mut TcpStream,
    content: &[u8],
    content_type: &str,
    filename: Option<&str>,
) -> io::Result<()> {
    stream
        .wa(format!(
            "HTTP/1.1 200 OK\r\nServer: ltheinrich.de stratos/{}\r\nContent-Type: {}\r\nContent-Length: {}{}\r\n\r\n",
            version(),
            content_type,
            content.len(),
            if let Some(filename) = filename {
                format!("\r\nContent-Disposition: attachment; filename=\"{}\"", filename)
            } else {
                String::new()
            }
        )
        .as_bytes())?;
    stream.wa(content)?;
    stream.wa(b"\r\n")?;
    Ok(())
}
