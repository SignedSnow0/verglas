use verglas::{
    core::Server,
    http::{Method, Response},
    router::{Route, RouterBuilder},
};

fn main() {
    let router = RouterBuilder::new()
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Get,
            handler: |_request| Response {
                status_code: 200,
                body: "Hello, GET!".to_string(),
            },
        })
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Post,
            handler: |_request| Response {
                status_code: 200,
                body: "Hello, POST!".to_string(),
            },
        })
        .build();

    let server = Server::new(vec!["127.0.0.1:80".parse().unwrap()], router);
    server.run();
}
