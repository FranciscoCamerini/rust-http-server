use crate::http::Request;
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
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                println!(
                                    "Received request to the following path: \n\n{}",
                                    request.path
                                )
                            }
                            Err(e) => {
                                println!("Failed to parse a request: {}", e);
                            }
                        },
                        Err(e) => {
                            print!("Failed to read from connection: {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}
