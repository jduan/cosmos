use std::net::TcpListener;
use zero2prod::run;

pub fn spawn_app() -> String {
    // Port 0 will make the OS choose a random port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to address");
    tokio::spawn(server);

    // return the local address to the caller
    format!("http://127.0.0.1:{}", port)
}
