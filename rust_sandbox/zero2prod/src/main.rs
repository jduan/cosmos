use std::net::TcpListener;
use zero2prod::run;

// This macro expands into a regular main function that starts an async runtime and
// drives it to completion.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
    run(listener)?.await
}
