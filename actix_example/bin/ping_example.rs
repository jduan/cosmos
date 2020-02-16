use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    // Callback for when the actor has been started
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("MyActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("MyActor is stopped");
    }
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// MyActor handles Ping messages
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");
        self.count += msg.0;
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let response = addr.send(Ping(10)).await;

    // handle() returns tokio handle
    println!("Result: {}", response.unwrap() == 20);

    System::current().stop();
}
