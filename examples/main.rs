use devonrex::prelude::*;
use std::{fs, thread};

// http://127.0.0.1:8080/
#[route(get,/)]
fn Index(request: Request) -> Response {
    let mut response = Response::default();
    if let Ok(html) = fs::read_to_string("./public/index.html") {
        response.body(html)
    }
    response
}
// http://127.0.0.1:8080/user/1
#[route(get,/user/<id>)]
fn Dynamic(request: Request) -> Response {
    let mut response = Response::default();
    response.body(
        request
            .parameters
            .get("id")
            .map_or("0".to_string(), |id| id.to_string()),
    );

    response
}
#[middleware(get,/)]
fn Midlewareprueba() -> State {
    let mut response = Response::default();
    response.body("hola mundo 123".to_string());
    // State::Response(response)
    State::Continue
}

fn main() {
    // let port = utils::find_port();
    // println!("http://127.0.0.1:{0}/", port);
    Rex::default()
        .add_routes(Index)
        .add_routes(Dynamic)
        .add_middleware(Midlewareprueba)
        .run();
}
