use client::Client;

use crate::statuscode;

use std::collections::HashMap;
#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status: statuscode::Status,
    pub headers: HashMap<String, String>,
    pub body: Body,
}
#[derive(Debug)]
pub enum FileHandle {
    // Save all file on the memory and send
    File(Vec<u8>),
    // ve her full control so she can manage the manual file.
    LargeFile(Box<fn(&mut client::Client)>),
}
#[derive(Debug)]

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

        let _ = match self.body {
            | Body::Text(e) => client.write(e.as_bytes()),
            | Body::File(FileHandle::File(v)) => client.write(&v.as_slice()),
            | Body::File(FileHandle::LargeFile(cb)) => {
                cb(client);
                Ok(())
            },
            | Body::None => Ok(()),
        };
    }
}
