/// This example shows you can have two actors where the result of the first actor
/// feeds into the second actor.
use actix::prelude::*;

/// An actor that sums up two numbers
struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

struct Value(usize, usize);

impl Message for Value {
    type Result = usize;
}

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}
/// An actor that displays a number
struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

struct Display(usize);

impl Message for Display {
    type Result = ();
}

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

#[actix_rt::main]
async fn main() {
    let sum_actor = SumActor {}.start();
    let display_actor = DisplayActor {}.start();

    for _i in 0..5 {
        // send message and get future for result
        let response = sum_actor.send(Value(34, 99)).await;
        if response.is_ok() {
            let _ = display_actor.send(Display(response.unwrap())).await;
        };
    }

    System::current().stop();
}
