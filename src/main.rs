#[allow(dead_code)]
mod http;
mod router;
mod server;

use env_logger;
use router::Router;
use server::Server;
use std::env;

fn setup_log() {
    env::set_var("RUST_LOG", "http_server");
    env_logger::init();
}

fn main() {
    setup_log();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Router);
}
