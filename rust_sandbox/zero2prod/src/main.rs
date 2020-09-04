use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// This macro expands into a regular main function that starts an async runtime and
// drives it to completion.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    run(listener)?.await
}
