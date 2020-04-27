use simple_http_server::ThreadPool;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    // If this buffer isn't large enough for a single request, we need to do buffer management
    // which is more complicated. We keep it simple for now.
    let mut buffer = [0; 1024];
    let _bytes_read = stream.read(&mut buffer).unwrap();

    let pattern = b"GET / HTTP/1.1\r\n";
    let sleep_pattern = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(pattern) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_pattern) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = std::fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
