use devonrex::{prelude::*, utils};
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

fn main() {
    let port = utils::find_port();
    println!("http://127.0.0.1:{0}/", port);
    Rex::new(port, 5)
        .add_routes(Index)
        .add_routes(Dynamic)
        .run();
}
