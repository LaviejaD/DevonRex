# Devon Rex

![devon rex cat](https://media.gettyimages.com/id/1222069589/es/foto/obedient-devon-rex-cat-sentado-en-la-caja-de-la-camada-en-la-sala-de-estar-foto-de-archivo.jpg?s=612x612&w=0&k=20&c=7ny3nJAThHNvpythuOafHPb2-dfSAZiTBR021m_8X_k=)

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/LaviejaD/DevonRex)

## Simple web framework

```RUST
use devonrex::prelude::*;
use std::{fs, thread};

#[route(get,/)]
fn Index(request: Request) -> Response {
    let mut response = Response::default();
    if let Ok(html) = fs::read_to_string("./public/index.html") {
        response.body(html)
    }

    response
}

fn main() {
    let port = 8080;
    println!("http://127.0.0.1:{0}/", port);
    Rex::new(port, 5).add_routes(Index).run();
}
```

