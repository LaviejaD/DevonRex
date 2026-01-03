// main lib
pub mod devon;
// utils
pub mod utils;
//other lib
pub use client;
pub use http;
pub use middleware;
pub use middleware_macro;
pub use router;
pub use router_macro;
// export all :)
pub mod prelude {
    pub use crate::devon::*;
    pub use middleware::*;
    pub use middleware_macro::*;

    pub use client::*;
    pub use http::*;
    pub use router::*;
    pub use router_macro::*;
    pub use std::thread;
}
