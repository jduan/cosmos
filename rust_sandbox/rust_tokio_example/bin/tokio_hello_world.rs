/// You can test this client by starting a server:
/// ncat -l 6142
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Created stream");

    let result = stream.write(b"hello world\n").await;
    println!("Wrote to stream; success = {:?}", result);

    let result = stream.write(b"hello world\n").await;
    println!("Wrote to stream; success = {:?}", result);

    Ok(())
}
