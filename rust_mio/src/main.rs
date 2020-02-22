use std::io;
use rust_mio::start_server;

fn main() -> io::Result<()> {
    env_logger::init();
    start_server()
}
