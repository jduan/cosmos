/// Create a ring of N actors. Send a message around the ring M times so that a total of M*N
/// messages are sent. Time how long this takes for different values of M and N.
/// You can achieve about 200k messages/second with 10k nodes and 100 rounds.
/// In contrast, Erlang can achieve 600k messages/second with the same setup.
use actix::prelude::*;
use std::time::SystemTime;
use structopt::StructOpt;

// A payload with a counter
struct Payload(usize);

impl Message for Payload {
    type Result = ();
}

struct Node {
    id: usize,
    limit: usize,
    next: Recipient<Payload>,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Handler<Payload> for Node {
    type Result = ();

    fn handle(&mut self, msg: Payload, _ctx: &mut Context<Self>) {
        if msg.0 >= self.limit {
            println!(
                "Actor {} reached limit of {} (payload was {})",
                self.id, self.limit, msg.0
            );
            System::current().stop();
            return;
        }

        // Some prime in order for different actors to report progress.
        // Large enough to print about once per second in debug mode.
        if msg.0 % 498_989 == 1 {
            println!(
                "Actor {} received message {} of {} ({:.2}%)",
                self.id,
                msg.0,
                self.limit,
                100.0 * msg.0 as f32 / self.limit as f32
            );
        }

        self.next
            .do_send(Payload(msg.0 + 1))
            .expect("Unable to send payload");
    }
}

#[derive(Debug, Clone, Copy, StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    /// number of nodes
    #[structopt(short = "n", long, default_value = "10000")]
    num_of_nodes: usize,
    /// number of times the message travels around the ring
    #[structopt(short = "m", long, default_value = "10")]
    num_of_times: usize,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let n_nodes = opt.num_of_nodes;
    let n_times = opt.num_of_times;
    let limit = n_nodes * n_times;
    let system = System::new("ring");

    println!("Setting up {} nodes", n_nodes);
    let setup = SystemTime::now();
    let node = Node::create(move |ctx| {
        let first_addr = ctx.address();
        let mut prev_addr = Node {
            id: 1,
            limit,
            next: first_addr.recipient(),
        }
        .start();

        for id in 2..n_nodes {
            prev_addr = Node {
                id,
                limit,
                next: prev_addr.recipient(),
            }
            .start();
        }

        Node {
            id: n_nodes,
            limit,
            next: prev_addr.recipient(),
        }
    });

    match setup.elapsed() {
        Ok(elapsed) => println!(
            "Time taken: {}.{:06} seconds",
            elapsed.as_secs(),
            elapsed.subsec_micros()
        ),
        Err(e) => eprintln!("An error occurred: {:?}", e),
    }

    let now = SystemTime::now();
    println!(
        "Sending start message and waiting for termination after {} messages ...",
        limit
    );
    let _req = node.send(Payload(1));

    match system.run() {
        Ok(_) => println!("Completed"),
        Err(e) => println!("An error occurred: {:?}", e),
    }
    match now.elapsed() {
        Ok(elapsed) => println!(
            "Time taken: {}.{:06} seconds ({} msg/second)",
            elapsed.as_secs(),
            elapsed.subsec_micros(),
            (n_nodes * n_times * 1_000_000) as u128 / elapsed.as_micros()
        ),
        Err(e) => eprintln!("An error occurred: {:?}", e),
    }

    Ok(())
}
