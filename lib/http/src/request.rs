use crate::method::Method;
use client::Client;
use std::{collections::HashMap, io::prelude::*};
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub parameters: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub client: Client,
}

impl Request {
    pub fn new(
        method: String,
        endpoint: String,
        http_version: String,
        headers: HashMap<String, String>,
        parameters: HashMap<String, String>,
        query: HashMap<String, String>,
        client: Client,
    ) -> Self {
        Self {
            method: Method::from_string(method),
            endpoint,
            http_version,
            headers,
            parameters,
            query,
            client,
        }
    }
    //read all bytes and return String
    pub fn read_all_to_text(&self) -> Result<String, ()> {
        let Some(length) = self.headers.get("Content-Length") else {
            return Err(());
        };
        let length: usize = match length.parse() {
            | Ok(e) => e,
            | Err(_) => 0,
        };
        let mut buff = Vec::<u8>::new();
        let mut tempb = [0u8; 1024];
        loop {
            if buff.len() == length {
                break;
            }
            let _ = self.client.read().read(&mut tempb);
            let _ = &buff.extend_from_slice(&tempb);
        }

        let r = String::from_utf8(buff).unwrap();
        Ok(r)
    }
    //  read all and return vec
    pub fn vec(&self) -> Vec<u8> {
        // self.client.read().read();
        Vec::new()
    }
}
