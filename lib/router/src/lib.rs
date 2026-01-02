use client::Client;
use http::{
    equal_url, get_url_params_and_value, has_dinamy_params, url_split, Method, Request, Response,
};

use std::{collections::HashMap, thread};

pub trait Route {
    fn run(&self, request: Request, client: Client) -> thread::JoinHandle<()>;
    fn endpoint(&self) -> (Method, String);
    fn callback(&self, request: Request) -> Response;
}

pub struct Routes {
    pub routes: HashMap<String, Box<dyn Route>>,
    pub routes_dinamy: HashMap<String, Box<dyn Route>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            routes_dinamy: HashMap::new(),
        }
    }
    pub fn insert(&mut self, route: impl Route + 'static) {
        let (method, endpoint) = route.endpoint();
        let format = format!("{} {}", method.to_string(), endpoint.to_lowercase());

        match has_dinamy_params(endpoint.clone()) {
            true => self.routes_dinamy.insert(format, Box::new(route)),
            false => self.routes.insert(format, Box::new(route)),
        };
    }

    pub fn get(&self, request: &mut Request) -> Option<&Box<dyn Route>> {
        #[allow(unused_assignments)]
        let mut result: Option<&Box<dyn Route>> = None;

        let get = format!(
            "{} {}",
            request.method.to_string(),
            request.endpoint.clone()
        );

        if let Some(r) = self.routes.get(&get) {
            return Some(r);
        } else {
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
}
