use client::Client;

use crate::statuscode;

use std::collections::HashMap;
pub struct Response {
    pub version: String,
    pub status: statuscode::Status,
    pub headers: HashMap<String, String>,
    pub body: Body,
}
pub enum FileHandle {
    File(Vec<u8>),
    LargeFile(Box<fn(client::Client)>),
}

pub enum Body {
    Text(String),
    File(FileHandle),
    None,
}

impl Response {
    pub fn default() -> Self {
        Response {
            version: String::from("HTTP/1.1"),
            status: crate::Status::Ok,
            headers: HashMap::new(),
            body: Body::None,
        }
    }

    pub fn text(&mut self, txt: String) {
        let length = txt.len().to_string();
        self.headers
            .insert("Contente-Type".to_string(), "text/plain".to_string());
        self.headers.insert("Content-Length".to_string(), length);
        self.body = Body::Text(txt);
    }

    pub fn html(&mut self, txt: String) {
        let length = txt.len().to_string();
        self.headers
            .insert("Contente-Type".to_string(), "text/html".to_string());
        self.headers.insert("Content-Length".to_string(), length);
        self.body = Body::Text(txt);
    }
    pub fn http(self, client: &mut Client) {
        let mut headers = String::new();
        headers.push_str(&format!("{} {}", self.version, self.status.to_string()));

        for (key, value) in &self.headers {
            headers.push_str(&format!("\r\n{}: {}", key, value))
        }

        headers.push_str(&format!("\r\n\r\n{}", ""));
        let _ = client.write(headers.as_bytes());

        match self.body {
            | Body::Text(e) => client.write(e.as_bytes()).unwrap(),
            | Body::File(FileHandle::File(v)) => client.write(&v.as_slice()).unwrap(),
            | Body::File(FileHandle::LargeFile(_)) => todo!("Implent manager for large file < 1gb"),
            | Body::None => (),
        }
    }
}
