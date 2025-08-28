use crate::statuscode;
//
use std::collections::HashMap;
pub struct Response {
    pub version: String,
    pub status: statuscode::Status,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn default() -> Self {
        Response {
            version: String::from("HTTP/1.1"),
            status: crate::Status::Ok,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn body(&mut self, body: String) {
        let length = body.len().to_string();
        self.headers.insert("Content-Length".to_string(), length);
        self.body = body;
    }

    pub fn http(&self) -> String {
        let mut http = String::new();
        http.push_str(&format!("{} {}", self.version, self.status.to_string()));

        for (key, value) in &self.headers {
            http.push_str(&format!("\r\n{}: {}", key, value))
        }

        http.push_str(&format!("\r\n\r\n{}", self.body));
        http
    }
}
