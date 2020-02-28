#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    static NTHREADS: i32 = 3;

    #[test]
    fn multiple_senders_one_receiver() {
        // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
        // where `T` is the type of the message to be transferred
        // (type annotation is superfluous)
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut children = Vec::new();

        for id in 0..NTHREADS {
            // The sender endpoint can be copied
            let thread_tx = tx.clone();

            // Each thread will send its id via the channel
            let child = thread::spawn(move || {
                // The thread takes ownership over `thread_tx`
                // Each thread queues a message in the channel
                thread_tx.send(id).expect("Failed to send message");

                // Sending is a non-blocking operation, the thread will continue
                // immediately after sending its message
                println!("thread {} finished", id);
            });

            children.push(child);
        }

        // Wait for the threads to complete any remaining work
        for child in children {
            child.join().expect("oops! the child thread panicked");
        }

        // Here, all the messages are collected
        let mut ids = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            // The `recv` method picks a message from the channel
            // `recv` will block the current thread if there are no messages available
            ids.push(rx.recv().expect("Expect an integer!"));
        }

        // Show the order in which the messages were sent
        println!("{:?}", ids);
        ids.sort();
        assert_eq!(ids, vec![0, 1, 2]);
    }
}
