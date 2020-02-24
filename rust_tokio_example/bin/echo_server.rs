extern crate rust_tokio_example;
use rust_tokio_example::echo_server_lib::echo_server;

#[tokio::main]
async fn main() {
    echo_server().await
}
