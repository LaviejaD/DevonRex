// use client::Client;
use http::{
    Method, Request, Response, equal_url, get_url_params_and_value, has_dinamy_params, url_split,
};

use std::collections::HashMap;

pub trait Middleware {
    fn endpoint(&self) -> (Method, String);
    fn callback(&self, request: Request) -> State;
    fn run(&self, request: Request) -> State;
}
#[derive(Clone)]
pub enum State {
    Continue,
    Response(Response),
}

pub struct Middlewares {
    pub routes: HashMap<String, Box<dyn Middleware>>,
    pub routes_dinamy: HashMap<String, Box<dyn Middleware>>,
}

impl Middlewares {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            routes_dinamy: HashMap::new(),
        }
    }
    pub fn insert(&mut self, route: impl Middleware + 'static) {
        let (method, endpoint) = route.endpoint();
        let format = format!("{} {}", method.to_string(), endpoint.to_lowercase());

        match has_dinamy_params(endpoint.clone()) {
            true => self.routes_dinamy.insert(format, Box::new(route)),
            false => self.routes.insert(format, Box::new(route)),
        };
    }

    pub fn get(&self, request: &mut Request) -> Option<&Box<dyn Middleware>> {
        #[allow(unused_assignments)]
        let mut result: Option<&Box<dyn Middleware>> = None;

        let get = format!(
            "{} {}",
            request.method.to_string(),
            request.endpoint.clone()
        );

        if let Some(r) = self.routes.get(&get) {
            return Some(r);
        }
        let u2 = url_split(get.to_lowercase());
        let mut keys = self.routes_dinamy.keys();

        if let Some(key) =
            keys.find(|&key| equal_url(url_split(key.clone().to_lowercase()), u2.clone()))
        {
            let u1 = url_split(key.clone().to_lowercase());
            get_url_params_and_value(u1, u2).iter().for_each(|(n, v)| {
                request.parameters.insert(n.clone(), v.clone());
            });

            result = self.routes_dinamy.get(key);
        }

        result
    }
}
