use std::thread::sleep;
use std::time::Duration;

pub async fn hello_world() {
    println!("hello, world!");
}

pub struct Song {
    pub name: String,
}

pub async fn learn_song() -> Song {
    println!("learn_song() is in thread: {:?}", std::thread::current());
    sleep(Duration::new(0.1 as u64, 1));
    Song {
        name: "Rain".to_string(),
    }
}

pub async fn sing_song(song: Song) {
    println!("sing_song() is in thread: {:?}", std::thread::current());
    sleep(Duration::new(0.1 as u64, 1));
    println!("I'm singing song: {}", song.name);
}

pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

pub async fn dance() {
    println!("dance() is in thread: {:?}", std::thread::current());
    sleep(Duration::new(0.1 as u64, 1));
    println!("I'm dancing now!");
}

/// The idea is that learn_and_sing, and dance can run concurrently. However, in this particular
/// case, both happen to run on the "main" thread so they actually run in sequence.
pub async fn have_fun() {
    let future1 = learn_and_sing();
    let future2 = dance();
    futures::join!(future2, future1);
}

#[cfg(test)]
mod tests {
    use crate::async_example::{have_fun, hello_world};
    use futures::executor::block_on;

    #[test]
    fn test_hello_world() {
        let future = hello_world();
        block_on(future);
    }

    #[test]
    fn test_have_fun() {
        block_on(have_fun());
    }
}
