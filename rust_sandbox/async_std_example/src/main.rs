use async_std::task::{sleep, spawn, Context, Poll};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

#[async_std::main]
async fn main() {
    // Here we spawn 3 async functions. This shows that all of them run in
    // the same main thread. You can adjust the number of sleepus we spawn here
    // and confirm that the number of threads won't change.
    let sleepus1 = spawn(sleepus());
    // let sleepus2 = spawn(sleepus());
    // let sleepus3 = spawn(sleepus());
    interruptus().await;
    sleepus1.await;
    // sleepus2.await;
    // sleepus3.await;
}

async fn sleepus() {
    for i in 1..=10 {
        println!("Sleepus {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

// This is identical to the sleepus() function above!
#[allow(dead_code)]
fn sleepus2() -> impl std::future::Future<Output = ()> {
    async {
        for i in 1..=10 {
            println!("Sleepus {}", i);
            sleep(Duration::from_millis(500)).await;
        }
    }
}

struct DoNothing;

impl Future for DoNothing {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

// This is identical to the sleepus() function above!
#[allow(dead_code)]
fn sleepus3() -> impl std::future::Future<Output = ()> {
    async {
        for i in 1..=10 {
            println!("Sleepus {}", i);
            sleep(Duration::from_millis(500)).await;
        }

        // async_std::future::ready(())
        DoNothing.await
    }
}

#[allow(dead_code)]
async fn sleepus4() {
    DoNothing.await
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}
