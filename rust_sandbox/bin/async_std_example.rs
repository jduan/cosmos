use factor::factor::factor;
use futures::executor::block_on;
use futures::join;
use futures::task::{Context, Poll};
use futures::Future;
use std::pin::Pin;

#[derive(Debug)]
struct Task {
    pub name: String,
    pub num: u32,
    pub count: u32,
}

impl Task {
    pub fn new(name: String, num: u32) -> Self {
        Self {
            name,
            num,
            count: 0,
        }
    }
}

impl Future for Task {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count > 10 {
            Poll::Ready(())
        } else {
            factor(self.num as i64);
            println!("The {} task progress {}", &self.name, self.count);
            self.get_mut().count += 1;
            cx.waker().clone().wake();

            Poll::Pending
        }
    }
}

async fn async_op() {
    let fast_task = Task::new("fast".to_string(), 123);
    let slow_task = Task::new("slow".to_string(), 43213200);
    join!(fast_task, slow_task);
}

fn main() {
    block_on(async_op());
}
