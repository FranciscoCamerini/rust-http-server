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
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    setup_log();

    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run(Router::new(public_path));
}
