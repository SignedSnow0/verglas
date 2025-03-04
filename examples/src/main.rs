use verglas::{
    core::Server,
    http::{cookie::CookieBuilder, response::ResponseBuilder, Method},
    router::{Route, RouterBuilder},
};

fn main() {
    let router = RouterBuilder::new()
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Get,
            handler: |request| {
                let cookie_counter = CookieBuilder::new().with_key("counter");

                let cookie_counter = match request.get_cookie("counter") {
                    Some(cookie) => {
                        let counter = match cookie.value.parse::<u32>() {
                            Ok(counter) => counter + 1,
                            Err(_) => 1,
                        };

                        cookie_counter.with_value(counter.to_string().as_str())
                    }
                    None => cookie_counter.with_value("1"),
                };

                let response = ResponseBuilder::new()
                    .empty()
                    .with_body("Hello, GET!")
                    .with_cookies(vec![cookie_counter.build()])
                    .build();

                println!("{}", String::from(&response));

                response
            },
        })
        .with_route(Route {
            path: "/".to_string(),
            method: Method::Post,
            handler: |_request| {
                ResponseBuilder::new()
                    .empty()
                    .with_body("Hello, POST!")
                    .build()
            },
        })
        .build();

    let server = Server::new(vec!["127.0.0.1:80".parse().unwrap()], router);
    server.run();
}
