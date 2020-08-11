use std::sync::mpsc::channel;
use std::time::Duration;
use threadpool::ThreadPool;

/// Start a thread pool of 4 workers. Send 8 jobs to a channel using the thread pool.
/// The "sleep" shows that the "tasks" submitted to the thread pool blocks the threads
/// that are executing the tasks.
fn synchronize_via_channel() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for i in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            let thread_id = std::thread::current().id();
            std::thread::sleep(Duration::from_secs(5));
            println!("send {} to channel from {:?}", i, thread_id);
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    println!("main thread: {:?}", std::thread::current().id());
    let sum = rx.iter().take(n_jobs).sum();
    assert_eq!(8, sum);
}

fn main() {
    println!("Hello, world!");
    synchronize_via_channel();
}
