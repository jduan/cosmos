/// This implements a very simple http server. Parsing of http request is done via http and
/// httparser libs. You can start the server and interact with it using curl:
///
///     curl -i http://localhost:8080/plaintext
///     curl -i http://localhost:8080/json
use bytes::BytesMut;
use futures::SinkExt;
use http::{HeaderValue, Request, Response, StatusCode};
use serde_derive::Serialize;
use std::error::Error;
use std::{env, fmt, io};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};

#[tokio::main]
/// TODO: understand how things work!
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let mut server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
    let mut incoming = server.incoming();
    while let Some(Ok(stream)) = incoming.next().await {
        println!("Got a new client from: {:#?}", stream.peer_addr());
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("Failed to process connection; error = {}", e);
            }
        });
    }

    Ok(())
}

/// Process requests from a single client.
async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, Http);
    while let Some(request) = transport.next().await {
        println!("Got a new request: {:#?}", request);
        match request {
            Ok(request) => {
                let response = respond(request).await?;
                transport.send(response).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

/// Send a response to the client.
/// Request: a type from the http crate. It has headers and body.
/// Response: a type from the http crate. It has headers and body.
async fn respond(req: Request<()>) -> Result<Response<String>, Box<dyn Error>> {
    let mut response = Response::builder();
    let body = match req.uri().path() {
        "/plaintext" => {
            response = response.header("Content-Type", "text/plain");
            "Hello, World!".to_string()
        }
        "/json" => {
            response = response.header("Content-Type", "application/json");
            #[derive(Serialize)]
            struct Message {
                message: &'static str,
            }
            serde_json::to_string(&Message {
                message: "Hello, World!",
            })?
        }
        _ => {
            response = response.status(StatusCode::NOT_FOUND);
            String::new()
        }
    };

    let response = response
        .body(body)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    Ok(response)
}

struct Http;

impl Decoder for Http {
    type Item = Request<()>;
    type Error = io::Error;

    /// Convert a request (bytes) into a Request object.
    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Request<()>>> {
        let mut headers = [None; 16];
        let (method, path, version, amt) = {
            let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
            let mut r = httparse::Request::new(&mut parsed_headers);
            let status = r.parse(src).map_err(|e| {
                let msg = format!("Failed to parse http request: {:?}", e);
                io::Error::new(io::ErrorKind::Other, msg)
            })?;
            let amt = match status {
                httparse::Status::Complete(amt) => amt,
                httparse::Status::Partial => return Ok(None),
            };

            let to_slice = |a: &[u8]| {
                let start = a.as_ptr() as usize - src.as_ptr() as usize;
                assert!(start < src.len());
                (start, start + a.len())
            };

            for (i, header) in r.headers.iter().enumerate() {
                let k = to_slice(header.name.as_bytes());
                let v = to_slice(header.value);
                headers[i] = Some((k, v));
            }

            (
                to_slice(r.method.unwrap().as_bytes()),
                to_slice(r.path.unwrap().as_bytes()),
                r.version.unwrap(),
                amt,
            )
        };

        if version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "only HTTP/1.1 accepted",
            ));
        }

        let data = src.split_to(amt).freeze();
        let mut builder = Request::builder();
        builder = builder.method(&data[method.0..method.1]);
        let s = data.slice(path.0..path.1);
        let s = unsafe { String::from_utf8_unchecked(Vec::from(s.as_ref())) };
        builder = builder.uri(s);
        builder = builder.version(http::Version::HTTP_11);

        for header in headers.iter() {
            let (k, v) = match *header {
                Some((ref k, ref v)) => (k, v),
                None => break,
            };

            let value = HeaderValue::from_bytes(data.slice(v.0..v.1).as_ref())
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "header decode error"))?;

            builder = builder.header(&data[k.0..k.1], value);
        }

        let req = builder
            .body(())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Some(req))
    }
}

/// Implementation of encoding an HTTP response into a `BytesMut`, basically
/// just writing out an HTTP/1.1 response.
impl Encoder<Response<String>> for Http {
    type Error = io::Error;

    /// Convert a Response object into a list of bytes.
    fn encode(&mut self, item: Response<String>, dst: &mut BytesMut) -> io::Result<()> {
        use chrono::Utc;
        use std::fmt::Write;

        write!(
            BytesWrite(dst),
            "\
             HTTP/1.1 {}\r\n\
             Server: Example\r\n\
             Content-Length: {}\r\n\
             Date: {}\r\n\
             ",
            item.status(),
            item.body().len(),
            Utc::now(),
        )
        .unwrap();

        for (k, v) in item.headers() {
            dst.extend_from_slice(k.as_str().as_bytes());
            dst.extend_from_slice(b": ");
            dst.extend_from_slice(v.as_bytes());
            dst.extend_from_slice(b"\r\n");
        }

        dst.extend_from_slice(b"\r\n");
        dst.extend_from_slice(item.body().as_bytes());

        return Ok(());

        // Right now `write!` on `Vec<u8>` goes through io::Write and is not
        // super speedy, so inline a less-crufty implementation here which
        // doesn't go through io::Error.
        struct BytesWrite<'a>(&'a mut BytesMut);

        impl fmt::Write for BytesWrite<'_> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.extend_from_slice(s.as_bytes());
                Ok(())
            }

            fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
                fmt::write(self, args)
            }
        }
    }
}
