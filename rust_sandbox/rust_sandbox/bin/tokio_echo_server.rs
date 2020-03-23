use futures::stream::StreamExt;
use tokio::net::TcpListener;
//use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());
                    // Handle this socket/client in the background
                    tokio::spawn(async move {
                        // split up the reading and writing parts of the socket
                        let (mut reader, mut writer) = socket.split();
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => println!("wrote {} bytes to {:?}", amt, socket.peer_addr()),
                            Err(err) => eprintln!("IO error: {:?}", err),
                        }
                    });
                }
                Err(err) => {
                    println!("accept error: {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    // Start the server and block this async fn until "server" spins down.
    server.await;
}
