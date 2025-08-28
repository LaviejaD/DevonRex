use crate::utils::find_port;
use client::Client;
use http::{parser_http_client, Request};
use middleware;
use router::{Route, Routes};
use std::{default, net::TcpListener};
use thread::ThreadManager;

pub struct Rex {
    port: u32,
    routes: Routes,
    middleware: middleware::Middlewares,
    threadmanager: ThreadManager,
}
impl default::Default for Rex {
    fn default() -> Self {
        Rex {
            port: find_port(),
            routes: Routes::new(),
            middleware: middleware::Middlewares::new(),
            threadmanager: ThreadManager::new(10),
        }
    }
}

impl Rex {
    pub fn new(port: u32, thread_size: usize) -> Self {
        Rex {
            port,
            routes: Routes::new(),
            middleware: middleware::Middlewares::new(),
            threadmanager: ThreadManager::new(thread_size),
        }
    }

    pub fn set_port(mut self, p: u32) -> Self {
        self.port = p;
        self
    }
    pub fn add_routes(&mut self, r: impl Route + 'static) -> &mut Self {
        self.routes.insert(r);
        self
    }
    pub fn add_middleware(&mut self, m: impl middleware::Middleware + 'static) -> &mut Self {
        self.middleware.insert(m);
        self
    }
    pub fn middleware_handle(&mut self, request: &mut Request) -> middleware::State {
        if let Some(middleware) = self.middleware.get(request) {
            return middleware.callback(request.clone());
        }
        middleware::State::Continue
    }
    pub fn routes_handle(&mut self, request: &mut Request, client: Client) {
        if let Some(route) = self.routes.get(request) {
            let r = route.run(request.clone(), client);
            self.threadmanager.add(r);
        }
    }
    pub fn run(&mut self) {
        if let Ok(lister) = TcpListener::bind(format!("127.0.0.1:{0}", self.port)) {
            for stream in lister.incoming() {
                match stream {
                    Ok(client_stream) => {
                        let client = Client::new(client_stream);
                        let mut request = parser_http_client(&client);

                        match self.middleware_handle(&mut request) {
                            middleware::State::Continue => self.routes_handle(&mut request, client),
                            middleware::State::Response(response) => {
                                println!("{}", response.http())
                            }
                        }
                    }

                    Err(e) => println!("Error {:#?}", e),
                }
            }
            //lister
        }
    }
}
