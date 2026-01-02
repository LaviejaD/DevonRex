use devonrex::prelude::*;
use std::{fs, thread};

// http://127.0.0.1:8080/
#[route(get,/)]
fn Index() -> Response {
    let mut response = Response::default();
    if let Ok(html) = fs::read_to_string("./public/index.html") {
        response.body(html)
    }
    response
}
// http://127.0.0.1:8080/<file>
#[route(get,/<file>)]
fn Public(request: Request) -> Response {
    let mut response = Response::default();
    let file = match request.parameters.get("file") {
        Some(e) => e,
        _ => "",
    };
    if let Ok(html) = fs::read_to_string(format!("./public/{}", file)) {
        response.body(html)
    } else {
        response.status = Status::NotFound;
    }

    response
}
#[middleware(get,/)]
fn MidlewareExample() -> State {
    let mut response = Response::default();
    response.body("hola mundo 123".to_string());
    // State::Response(response)
    State::Continue
}

fn main() {
    Rex::default()
        .set_port(33147)
        .add_routes(Index)
        .add_routes(Public)
        .add_middleware(MidlewareExample)
        .run();
}
