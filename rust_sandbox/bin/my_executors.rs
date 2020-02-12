//use futures::channel::oneshot;
//use std::future::Future;
//use std::pin::Pin;
//use std::sync::atomic::AtomicUsize;
//use std::sync::{Arc, Mutex};
//
//type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;
//
//struct Task {
//    state: AtomicUsize,
//    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
//}

// A queue that holds scheduled tasks
//static QUEUE: Lazy<channel::Sender<Arc<Task>>> = Lazy::new(|| {
//    // Create a queue
//    let (sender, receiver) = channel::unbounded::<Arc<Task>>();
//
//    // Spawn executor threads the first time the queue is created
//});

//fn spawn<F, R>(future: F) -> JoinHandle<R>
//where
//    F: Future<Output = R> + Send + 'static,
//    R: Send + 'static,
//{
//    // Wrap the future into one that sends the output to a channel
//    let (s, r) = oneshot::channel();
//    let future = async move {
//        let _ = s.send(future.await);
//    };
//
//    // Create a task and schedule it for execution
//    let task = Arc::new(Task {
//        state: AtomicUsize::new(0),
//        future: Mutex::new(Box::pin(future)),
//    });
//
//    QUEUE.send(task).unwrap();
//
//    // Return a join handle that retrieves the output of the future
//    Box::pin(async { r.await.unwrap() })
//}
//
//fn main() {
//    futures::executor::block_on(async {
//        //        let handle = spawn(async { 1 + 2 });
//        assert_eq!(handle.await, 3);
//    })
//}
