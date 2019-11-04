use std::thread;
use std::time::Duration;
// mpsc stands for "multiple producers, single consumer"
use std::sync::mpsc;

pub fn run() {
    create_thread();
    move_ownership();
    message_passing();
    message_passing2();
    multiple_producers();
}

fn create_thread() {
    println!("===== create_thread =====");
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
    println!("===== move_ownership =====");
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

fn message_passing() {
    println!("===== message_passing =====");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        // send returns a Result type
        tx.send(val).unwrap();

        // The line below doesn't compile! We try to print val after sending it down the channel.
        // Allowing that would be a bad idea: once the value has been sent to another thread, that
        // thread could modify or drop it before we try to use the value again. Potentially, the
        // other thread's modificiation could cause errors or unexpected results due to
        // inconsistent or nonexistent data.
        // println!("val is {}", val);
    });

    // recv is a blocking call and it returns a Result type
    // try_recv is the non-blocking version
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn message_passing2() {
    println!("===== message_passing2 =====");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx can be used as an iterator
    for received in rx {
        println!("Got {}", received);
    }
}

fn multiple_producers() {
    println!("===== multiple_producers =====");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx can be used as an iterator
    for received in rx {
        println!("Got {}", received);
    }
}
