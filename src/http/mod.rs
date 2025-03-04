use std::collections::HashMap;

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

// https://datatracker.ietf.org/doc/html/rfc2616#section-5\
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub cookies: Vec<RequestCookie>,
}

#[derive(Debug)]
pub struct Cookie {
    pub key: String,
    pub value: String,
    pub secure: bool,
    pub http_only: bool,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<String>,
    pub max_age: Option<String>,
    pub same_site: Option<String>,
}

#[derive(Debug)]
pub struct RequestCookie {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
    pub cookies: Vec<Cookie>,
}
