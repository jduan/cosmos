use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A job is an arbitrary function.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// The ThreadPool object uses a "channel" to send "jobs" (which are just functions) to
/// thread pool Workers. The workers get the "jobs" by reading from the "channel" and execute
/// the them.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "ThreadPool size can't be 0");
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // recv() blocks if there's no job yet
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);

            // Execute the actual task by calling the job function.
            job();
        });

        Self { id, thread }
    }
}
