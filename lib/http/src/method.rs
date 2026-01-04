#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
    Get,
    Post,
    Put,
    Head,
    Delete,
    Trace,
    Connect,
    Custom(String),
    None,
}

impl Method {
    pub fn to_string(&self) -> String {
        let r = match self {
            | Method::Get => "GET",
            | Method::Post => "POST",
            | Method::Put => "PUT",
            | Method::Head => "HEAD",
            | Method::Trace => "TRACE",
            | Method::Delete => "DELETE",
            | Method::Connect => "CONNECT",
            | Method::Custom(e) => e,
            | Method::None => "None",
        };
        r.to_string()
    }
    pub fn from_string(m: String) -> Self {
        match m.to_lowercase().as_str() {
            | "get" => Method::Get,
            | "post" => Method::Post,
            | "put" => Method::Put,
            | "head" => Method::Head,
            | "delete" => Method::Delete,
            | "trace" => Method::Trace,
            | "connect" => Method::Connect,
            | "none" => Method::None,
            | e if e.is_empty() => Method::None,
            | e => Method::Custom(e.to_string()),
        }
    }
}
