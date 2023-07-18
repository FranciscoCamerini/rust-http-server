use crate::http::{Request, Response, StatusCode};
use log::{info, warn};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                                Ok(request) => {
                                    dbg!(request);

                                    let response = Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Hello!</h1>".to_string()),
                                    );
                                    response
                                }
                                Err(e) => {
                                    warn!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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
