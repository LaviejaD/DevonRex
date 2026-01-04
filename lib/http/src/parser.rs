use crate::request;
use client::Client;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
pub fn parser_http_request(client: Client) -> Option<request::Request> {
    let args = r"\r\n\r\n";
    let args_len = args.len();
    let mut buffer = BufReader::with_capacity(args_len, client.read());

    let mut raw_request = String::new();
    let mut body = Vec::new();
    let mut is_header = true;
    let mut last = Vec::new();

    loop {
        match buffer.fill_buf() {
            | Ok(buf) => {
                let mut byts = buf.to_vec();
                let bytslen = byts.len();
                let mut e = 0;
                if args_len == bytslen && last.len() == bytslen {
                    if last[0] == byts[0] {
                        for i in 0..bytslen {
                            if last[i] == byts[i] {
                                e += 1;
                            }
                        }
                    }
                }
                last = byts.clone();
                if e == bytslen {
                    break;
                }
                if is_header {
                    match String::from_utf8(byts.clone()) {
                        | Ok(t) => {
                            if t == args {
                                is_header = false
                            }
                        },
                        | Err(_) => {
                            is_header = false;
                        },
                    }
                }
                if is_header {
                    if let Ok(txt_raw) = String::from_utf8(byts.clone()) {
                        raw_request.push_str(&txt_raw.to_owned());
                    } //txt_raw.split(&args).collect();
                } else {
                    body.append(&mut byts);
                }

                buffer.consume(args_len);

                if args_len > bytslen {
                    break;
                }
            },
            | Err(e) => {
                println!("{e}");
                break;
            },
        }
    }
    parser_http_request_raw(raw_request, client)
}
pub fn parser_http_request_raw(raw_request: String, client: Client) -> Option<request::Request> {
    let mut headers = HashMap::new();
    let mut query = HashMap::new();

    let mut raw_headers: Vec<&str> = Vec::new();
    let mut method = String::new();
    let mut endpoint = String::new();
    let mut version = String::new();
    #[allow(unused)]
    let mut url = String::new();

    let mut lines: Vec<_> = raw_request.lines().collect();

    if lines.len() > 0 {
        raw_headers.append(&mut lines.remove(0).split(" ").collect::<Vec<_>>());

        match raw_headers.get(0) {
            | Some(r) => method = r.to_string(),
            | None => return None,
        }
        match raw_headers.get(1) {
            | Some(s) => url = s.to_string(),
            | None => return None,
        };
        match raw_headers.get(2) {
            | Some(r) => {
                if let Some(r2) = r.to_string().split("/").collect::<Vec<_>>().pop() {
                    version = r2.to_string()
                } else {
                    return None;
                }
            },
            | None => return None,
        };

        for header in lines {
            if header.is_empty() {
                break;
            }
            let header_split: Vec<_> = header.split(": ").collect();

            if header_split.len() == 2 {
                headers.insert(header_split[0].to_string(), header_split[1].to_string());
            }
        }

        let url_raw: Vec<&str> = url.split("?").collect();
        endpoint = url_raw[0].to_string();

        match url_raw.get(1) {
            | Some(query_) => {
                for q in query_.split("&") {
                    let _query: Vec<_> = q.split("=").collect();
                    query.insert(_query[0].to_string(), _query[1].to_string());
                }
            },
            | None => (),
        }
    }

    let r = request::Request::new(
        method,
        endpoint,
        version,
        headers,
        HashMap::new(),
        query,
        client,
    );

    if r.method.to_string() != "None" {
        return Some(r);
    } else {
        return None;
    }
}
