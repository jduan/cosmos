/// For CPU bound workloads, you can use a SyncArbiter to run multiple instances of an Actor
/// in parallel (using a pool of OS threads).
/// It's important to note a SyncArbiter can only host a single type of Actor. This means you need
/// to create a SyncArbiter for each type of Actor you want to run in this manner.
use actix::prelude::*;
use factor::factor::factor;

struct MySyncActor;

impl Actor for MySyncActor {
    type Context = SyncContext<Self>;
}

struct FactorRequest(usize);

impl Message for FactorRequest {
    type Result = usize;
}

impl Handler<FactorRequest> for MySyncActor {
    type Result = usize;

    fn handle(&mut self, msg: FactorRequest, _ctx: &mut Self::Context) -> Self::Result {
        println!("Got request for factoring {}", msg.0);
        let res = factor(msg.0 as i64).len();
        println!("Done factoring {}", msg.0);
        res
    }
}

#[actix_rt::main]
async fn main() {
    // If you change the threads from 3 to 1, all the requests will be serialized. So you
    // won't leverage multiple CPU cores. That will increase the time from 14 seconds to 28 seconds.
    // When there are 3 threads, all the requests will be handled concurrently on different threads.
    let addr = SyncArbiter::start(3, || MySyncActor);

    // send message and get future for result
    let req1 = addr.send(FactorRequest(100_000_000));
    let req2 = addr.send(FactorRequest(200_000_000));
    let req3 = addr.send(FactorRequest(300_000_000));

    println!("Result 1: {:?}", req1.await.unwrap());
    println!("Result 2: {:?}", req2.await.unwrap());
    println!("Result 3: {:?}", req3.await.unwrap());

    System::current().stop();
}
