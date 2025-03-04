use verglas::{
    core::Server,
    http::{Cookie, Method, Response},
    router::{Route, RouterBuilder},
};

fn main() {
    let router = RouterBuilder::new()
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Get,
            handler: |request| {
                let cookie_counter = match request.get_cookie("counter") {
                    Some(cookie) => {
                        let counter = match cookie.value.parse::<u32>() {
                            Ok(counter) => counter + 1,
                            Err(_) => 1,
                        };

                        Cookie::new("counter", counter.to_string().as_str())
                    }
                    None => Cookie::new("counter", "1"),
                };

                let mut response = Response {
                    status_code: 200,
                    body: "Hello, GET!".to_string(),
                    cookies: Default::default(),
                };

                response.add_cookie(cookie_counter);

                println!("{}", String::from(&response));

                response
            },
        })
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Post,
            handler: |_request| Response {
                status_code: 200,
                body: "Hello, POST!".to_string(),
                cookies: Default::default(),
            },
        })
        .build();

    let server = Server::new(vec!["127.0.0.1:80".parse().unwrap()], router);
    server.run();
}
