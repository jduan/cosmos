use std::thread;
use std::time::Duration;

pub fn run() {
    create_thread();
    move_ownership();
}

fn create_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait for the spawned thread to finish
    handle.join().unwrap();
}

fn move_ownership() {
    let v = vec![1, 2, 3];
    // "move" says the spawned thread will own all the variables in the closure that are referenced
    // by the thread
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // The line below doesn't compile because the ownership of v has been moved to the spawned
    // thread.
    // println!("Can't access the vector anymore: {:?}", v);

    handle.join().unwrap();
}
