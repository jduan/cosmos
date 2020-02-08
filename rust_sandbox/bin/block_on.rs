/// See this blog post: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

use crossbeam::sync::Parker;
use futures::channel::oneshot;
use std::pin::Pin;

/// Runs a future to completion on the current thread. This function will block
/// the caller until the given future has completed.
#[allow(dead_code)]
fn block_on<F: Future>(future: F) -> F::Output {
    // Pin the future on the stack.
    pin_utils::pin_mut!(future);

    // Create a waker that unparks this thread.
    let thread = thread::current();
    let waker = async_task::waker_fn(move || thread.unpark());

    // Create the task context.
    let cx = &mut Context::from_waker(&waker);

    // Keep polling the future until completion.
    loop {
        println!("polling the future");
        // The context cx is passed to the future's poll. When the future is ready to make
        // progress, it will call the waker inside the context and the waker will call
        // "thread.unpack" to wake up the current thread!
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
        println!("done polling");
    }
}

/// Use Parker from crossbeam.
#[allow(dead_code)]
fn block_on2<F: Future>(future: F) -> F::Output {
    // Pin the future on the stack.
    pin_utils::pin_mut!(future);

    // Create a waker that unparks this thread.
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || unparker.unpark());

    // Create the task context.
    let cx = &mut Context::from_waker(&waker);

    // Keep polling the future until completion.
    loop {
        println!("polling the future");
        // The context cx is passed to the future's poll. When the future is ready to make
        // progress, it will call the waker inside the context and the waker will call
        // "thread.unpack" to wake up the current thread!
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
        println!("done polling");
    }
}

// Creating Paker and Waker is not free -- both of which incur the cost of an allocation.
// This function caches them in thread-local storage.
#[allow(dead_code)]
fn block_on3<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    thread_local! {
        static CACHE: (Parker, Waker) = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            (parker, waker)
        }
    }

    CACHE.with(|(parker, waker)| {
        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}

struct Yields(u32);

impl Future for Yields {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// The output should look like:
/// polling the future
/// Awaiting...
/// done polling
/// polling the future
/// Hell, world!
fn poll_future1() {
    let (s, r) = oneshot::channel();

    // Spawn a thread that will send a message through the channel.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        s.send("Hello, world!").unwrap();
    });

    // Block until the message is received.
    //    let msg = block_on(async {
    //    let msg = block_on2(async {
    let msg = block_on3(async {
        println!("Awaiting...");
        r.await.unwrap()
    });
    println!("{}", msg);
}

fn poll_future2() {
    // The future will be polled 6 times. The first 5 times, the poll would return Pending.
    // The 6th time, it would return Ready.
    block_on(Yields(5));
}

fn main() {
    poll_future1();
    poll_future2();
}
