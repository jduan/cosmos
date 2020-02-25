use rust_mio_example::start_server;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();
    start_server()
}
