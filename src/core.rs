use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use crate::{
    http::{Request, Response},
    router::Router,
};

#[derive(Debug)]
struct Connection {
    thread: std::thread::JoinHandle<()>,
}

pub struct Server {
    listener: TcpListener,
    connections: Vec<Connection>,
    router: Arc<Router>,
}

impl Server {
    pub fn new(addresses: Vec<SocketAddr>, router: Router) -> Self {
        Self {
            listener: TcpListener::bind(&addresses[..]).unwrap(),
            connections: Vec::new(),
            router: Arc::new(router),
        }
    }

    pub fn run(mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = self.router.clone();

                    self.connections.push(Connection {
                        thread: thread::spawn(move || Self::handle_connection(router, stream)),
                    });
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {e}");
                    continue;
                }
            }

            self.connections
                .retain(|connection| !connection.thread.is_finished());
        }
    }

    fn handle_connection(router: Arc<Router>, mut stream: TcpStream) {
        println!(
            "Connection established with {}!",
            stream.peer_addr().unwrap()
        );

        loop {
            match Request::try_from(&mut stream) {
                Ok(request) => {
                    let response = router.dispatch(&request);

                    stream.write_all(String::from(response).as_bytes()).unwrap();

                    match request.get_header("Connection") {
                        Some(connection) if connection == "close" => break,
                        _ => {}
                    }
                }
                Err(e) => {
                    let mut response = Response::internal_server_error();
                    response.body = e.to_string();

                    stream.write_all(String::from(response).as_bytes()).unwrap();
                }
            }
        }

        println!("Connection closed!");
    }
}
