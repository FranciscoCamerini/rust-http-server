use crate::http::{ParseError, Request, Response, StatusCode};
use log::{info, warn};
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bas_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        info!("Initializing server.");
        let listener = TcpListener::bind(&self.addr).unwrap();
        info!(
            "Server successfully initialized. Listening for connections on {}",
            self.addr
        );

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bas_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to read from connection: {}", e)
                        }
                    };
                }
                Err(e) => {
                    warn!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}
