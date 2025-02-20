use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use verglas::http::{Request, Response};

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = BufReader::new(&stream);

        let request = reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<String>();

        let request: Request = request.as_str().try_into().unwrap();

        println!("{request:#?}");

        let response = Response {
            status_code: 200,
            body: "".to_string(),
        };

        let response: String = response.into();
        stream.write_all(response.as_bytes()).unwrap();
    }
}
