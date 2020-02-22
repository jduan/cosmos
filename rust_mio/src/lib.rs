use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

pub fn start_server() -> io::Result<()> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::new();
    // Map of `Token` -> `Vec<u8>`.
    let mut pending_data = HashMap::new();
    // Unique token for each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 9000");
    println!("You'll see our welcome message and anything you type we'll be printed here.");

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Received an event for the TCP server socket.
                    // Accept an connection.
                    let (mut connection, address) = server.accept()?;
                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;

                    connections.insert(token, connection);
                    let data: Vec<u8> = Vec::with_capacity(4096);
                    pending_data.insert(token, data);
                }
                token => {
                    // (maybe) received an event for a TCP connection.
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        let data = pending_data.get_mut(&token).unwrap();
                        handle_connection_event(connection, data, event)?
                    } else {
                        // Sporadic events happen.
                        false
                    };
                    if done {
                        connections.remove(&token);
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
fn handle_connection_event(
    connection: &mut TcpStream,
    data: &mut Vec<u8>,
    event: &Event,
) -> io::Result<bool> {
    if event.is_writable() {
        println!("Connection is ready for write again");
        if data.is_empty() {
            println!("Nothing to write to the client");
            return Ok(false);
        }

        write_to_socket(connection, data)?;
    }

    if event.is_readable() {
        println!("Connection is ready for read again");
        let mut connection_closed = false;
        // We can (maybe) read from the connection.
        loop {
            let mut buf = [0; 256];
            match connection.read(&mut buf) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    data.extend_from_slice(&buf[..n]);
                    println!(
                        "Appending {} bytes to the data buffer. Buffer size now: {}",
                        n,
                        data.len()
                    );
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        write_to_socket(connection, data)?;

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn write_to_socket(connection: &mut TcpStream, data: &mut Vec<u8>) -> io::Result<()> {
    // We can (maybe) write to the connection.
    match connection.write(data) {
        // We want to write the entire `DATA` buffer in a single go. If we
        // write less we'll return a short write error (same as
        // `io::Write::write_all` does).
        Ok(n) if n < data.len() => Err(io::ErrorKind::WriteZero.into()),
        Ok(_) => {
            data.clear();
            Ok(())
        }
        // Would block "errors" are the OS's way of saying that the
        // connection is not actually ready to perform this I/O operation.
        Err(ref err) if would_block(err) => Ok(()),
        // Got interrupted (how rude!), we'll try again.
        Err(ref err) if interrupted(err) => write_to_socket(connection, data),
        // Other errors we'll consider fatal.
        Err(err) => Err(err),
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
