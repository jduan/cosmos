use mio::{EventLoop, io, buf};

fn main() {
    start().assert("The event loop could not be started");
}

fn start() -> MioResult<EventLoop> {

}
