use client::Client;
use http::{parser_http_client, Request, Response, Status};
use middleware;
use router::{Route, Routes};
use std::io::ErrorKind;
use std::{
    default,
    net::TcpListener,
    sync::{LazyLock, Mutex},
};
use thread::ThreadManager;

static DEVONREX_GLOBAL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

pub struct Rex {
    port: u16,
    routes: Routes,
    middleware: middleware::Middlewares,
    threadmanager: ThreadManager,
}
impl default::Default for Rex {
    fn default() -> Self {
        Rex {
            port: 0,
            routes: Routes::new(),
            middleware: middleware::Middlewares::new(),
            threadmanager: ThreadManager::new(10),
        }
    }
}

impl Rex {
    pub fn new(port: u16, thread_size: usize) -> Self {
        Rex {
            port,
            routes: Routes::new(),
            middleware: middleware::Middlewares::new(),
            threadmanager: ThreadManager::new(thread_size),
        }
    }

    pub fn set_port(mut self, p: u16) -> Self {
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
    pub fn middleware_handle(&mut self, request: &mut Request, mut client: Client) {
        if let Some(middleware) = self.middleware.get(request) {
            if let middleware::State::Response(response) = middleware.callback(request.clone()) {
                // let mut r = response.clone();
                // r.status = Status::NotFound;
                println!("{}", response.http());
                client.write(response.http().as_bytes()).unwrap();
                client.close().unwrap();
            } else {
                self.routes_handle(request, client);
            }
        }
    }

    pub fn routes_handle(&mut self, request: &mut Request, mut client: Client) {
        if let Some(route) = self.routes.get(request) {
            let r = route.run(request.clone(), client);
            self.threadmanager.add(r);
        } else {
            // println!("prueba");
            let mut r = Response::default();
            r.status = Status::NotFound;
            client.write(r.http().as_bytes()).unwrap();
            client.close().unwrap();
        }
    }
    pub fn stop() {
        let mut devorex_global = DEVONREX_GLOBAL.lock().unwrap();
        *devorex_global = false;
    }

    pub fn run(&mut self) {
        let tl = match self.port {
            p if p > 0 => TcpListener::bind(format!("127.0.0.1:{0}", self.port)),
            _ => TcpListener::bind("127.0.0.1:0"),
        };

        if let Ok(lister) = tl {
            if let Ok(addr) = lister.local_addr() {
                self.port = addr.port();
            }
            lister.set_nonblocking(true).unwrap();
            println!("http://127.0.0.1:{0}", self.port);
            *DEVONREX_GLOBAL.lock().unwrap() = true;

            //
            'main: loop {
                match lister.accept() {
                    Ok((client_stream, _)) => {
                        let client = Client::new(client_stream);
                        let mut request = parser_http_client(&client);

                        self.middleware_handle(&mut request, client);
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // No incoming connection to process yet, continue the loop or sleep briefly
                        // A busy loop consumes CPU, so a small sleep or a more sophisticated event loop is recommended
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => println!("{}", e),
                }

                if !*DEVONREX_GLOBAL.lock().unwrap() {
                    break 'main;
                }
            }

            println!("dev final {}", *DEVONREX_GLOBAL.lock().unwrap());
            self.threadmanager.join();
            // for stream in lister.incoming() {
            //     match stream {
            //         Ok(client_stream) => {
            //             let client = Client::new(client_stream);
            //             let mut request = parser_http_client(&client);

            //             self.middleware_handle(&mut request, client);
            //         }

            //         Err(e) => println!("Error {:#?}", e),
            //     }
            // }
            //if lister
        }
    }
}
