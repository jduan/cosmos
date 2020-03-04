use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let sleepus = spawn(sleepus);
    let interruptus = spawn(interruptus);

    sleepus.join().unwrap();
    interruptus.join().unwrap();
}

fn sleepus() {
    for i in 1..=10 {
        println!("Sleepus {}", i);
        sleep(Duration::from_millis(500));
    }
}

fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000));
    }
}
