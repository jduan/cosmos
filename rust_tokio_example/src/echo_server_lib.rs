use futures::stream::StreamExt;
use tokio::net::TcpListener;

pub async fn echo_server() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    // spawn a new task/future to handle this connection in the background
                    // so the server can accept more connections
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();

                        // tokio::io::copy copies all data read from the socket back to the socket
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => println!("wrote {} bytes", amt),
                            Err(err) => eprintln!("IO error {:?}", err),
                        }
                    });
                }
                Err(err) => {
                    println!("accept error {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");
    server.await;
}
