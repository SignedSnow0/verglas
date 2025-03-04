pub mod cookie;
pub mod request;
pub mod response;
pub mod types;

/// The Mehtod enum represents an HTTP method as defined in [RFC 2616](https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1)
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

#[derive(Debug)]
pub enum Version {
    Http1_0,
    Http1_1,
}

#[derive(Debug)]
pub struct Uri {
    pub path: String,
    pub attributes: Vec<(String, String)>,
}
