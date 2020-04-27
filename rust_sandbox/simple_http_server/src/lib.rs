use crate::Message::NewJob;
use crate::Message::Terminate;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A job is an arbitrary function.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// We need the ability to send a Terminate message so workers can break out of the infinite loop
/// and shut themselves down.
enum Message {
    NewJob(Job),
    Terminate,
}

/// The ThreadPool object uses a "channel" to send "jobs" (which are just functions) to
/// thread pool Workers. The workers get the "jobs" by reading from the "channel" and execute
/// the them.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Waiting for worker {} to shut down", worker.id);
            // Option.take() takes the value out of the option, leaving a None in its place.
            // We need to do this because we have a worker ref here but "join()" wants to take
            // ownership of the handle.
            if let Some(handle) = worker.thread.take() {
                handle.join().unwrap();
            }
        }

        println!("All workers have been shut down!");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Loop until receiving a Terminate message.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // recv() blocks if there's no job yet
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                // Execute the actual task by calling the job function.
                NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job()
                }
                Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
