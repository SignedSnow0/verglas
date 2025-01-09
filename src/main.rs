mod method;
mod router;
use crate::{method::Method, router::Router};
use router::RequestHandler;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut router = Router::<String>::new();

    router.add_get_endpoint("/", IndexHandler);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &router);
    }
}

fn handle_connection(mut stream: TcpStream, router: &Router<String>) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let method = Method::try_from(request_line.as_str()).unwrap();

    println!("Received request: {method:#?}");

    let (status_line, body) = match method {
        Method::Get(url) => match router.get_get_endpoint(&url) {
            Some(handler) => match handler.handle_request() {
                Some(body) => ("HTTP/1.1 200 OK".to_string(), body.into()),
                None => (
                    "HTTP/1.1 500 Internal Server Error".to_string(),
                    "".to_string(),
                ),
            },
            None => {
                let status_line = "HTTP/1.1 404 NOT FOUND";
                let body = fs::read_to_string("web_app/errors/404.html").unwrap();

                (status_line.to_string(), body)
            }
        },
        _ => {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let body = fs::read_to_string("web_app/errors/404.html").unwrap();

            (status_line.to_string(), body)
        }
    };

    let length = body.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}

struct IndexHandler;
impl RequestHandler for IndexHandler {
    type Body = String;

    fn handle_request(&self) -> Option<Self::Body> {
        Some(fs::read_to_string("web_app/index.html").unwrap())
    }
}

struct LoginHandler;
impl RequestHandler for LoginHandler {
    type Body = String;

    fn handle_request(&self) -> Option<Self::Body> {
        None
    }
}
