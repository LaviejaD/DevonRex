use devonrex::prelude::*;
use std::fs;

// http://127.0.0.1:8080/
#[route(get,/)]
fn Index() -> Response {
    let mut response = Response::default();
    if let Ok(html) = fs::read_to_string("./public/index.html") {
        response.html(html)
    }
    response
}
// http://127.0.0.1:8080/<file>
#[route(get,/<file>)]
fn Public(request: Request) -> Response {
    let mut response = Response::default();
    match request.parameters.get("file") {
        | Some(file) => {
            if let Ok(r) = fs::read_to_string(format!("./public/{}", file)) {
                response.text(r)
            }
        },
        | _ => response.status = Status::NotFound,
    }

    response
}
#[middleware(get,/)]
fn MidlewareExample() -> State {
    // let mut response = Response::default();
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
