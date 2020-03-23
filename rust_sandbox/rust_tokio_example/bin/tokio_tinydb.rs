/// You can interact with this server using nc:
///
/// nc localhost 8080
///
/// An example session:
///
/// GET foo
/// foo = bar
/// GET bar
/// error: no key found: bar
/// SET bar my awesome string
/// set bar = `my awesome string`, previous: None
/// GET bar
/// bar = my awesome string
/// get bar
/// error: Unknown command: get
/// SET foo tokio
/// set foo = `tokio`, previous: Some("bar")
/// GET foo
/// foo = tokio
use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

/// The in-memory database shared by all clients.
struct Database {
    map: Mutex<HashMap<String, String>>,
}

/// Requests that clients can send
enum Request {
    Get { key: String },
    Set { key: String, value: String },
}

impl Request {
    fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET must be followed by a key")?;
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("SET") => {
                let key = parts.next().ok_or("SET must be followed by a key")?;
                let value = parts.next().ok_or("SET needs a value")?;
                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("Unknown command: {}", cmd)),
            None => Err("Empty input".into()),
        }
    }
}

enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        // There may not be a previous value, hence the Option type.
        previous: Option<String>,
    },
    Error {
        msg: String,
    },
}

impl Response {
    fn serialize(&self) -> String {
        match *self {
            Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}

fn handle_request(line: &str, db: &Arc<Database>) -> Response {
    let request = match Request::parse(line) {
        Ok(request) => request,
        Err(err) => return Response::Error { msg: err },
    };

    // This lock of the Mutex gets released automatically when it's dropped at the end of this
    // function.
    let mut db = db.map.lock().unwrap();
    match request {
        Request::Get { key } => match db.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.to_string(),
            },
            None => Response::Error {
                msg: format!("no key found: {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = db.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let mut initial_db = HashMap::new();
    initial_db.insert("foo".to_string(), "bar".to_string());
    let db = Arc::new(Database {
        map: Mutex::new(initial_db),
    });

    loop {
        match listener.accept().await {
            Ok((mut socket, _addr)) => {
                let db = db.clone();
                socket.write(b"Welcome to tinydb!").await?;

                // Spawn a task to handle this client concurrently.
                tokio::spawn(async move {
                    // Our protocol is line based so we use a LinesCodec to convert our stream
                    // of bytes into a stream of lines.
                    let mut lines = Framed::new(socket, LinesCodec::new());

                    while let Some(result) = lines.next().await {
                        match result {
                            Ok(line) => {
                                let response = handle_request(&line, &db).serialize();
                                if let Err(e) = lines.send(response.as_str()).await {
                                    println!("Error on sending response; error = {:?}", e);
                                }
                            }
                            Err(e) => {
                                println!("Error on decoding from socket; error = {:?}", e);
                            }
                        }
                    }

                    // The connection will be closed at this point as "lines.next()" has returned None
                });
            }
            Err(e) => println!("Error accepting socket; error = {:?}", e),
        }
    }
}
