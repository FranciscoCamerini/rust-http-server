#[allow(dead_code)]
mod http;
mod server;
mod website_handler;

use env_logger;
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

fn setup_log() {
    env::set_var("RUST_LOG", "http_server");
    env_logger::init();
}

fn main() {
    setup_log();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
