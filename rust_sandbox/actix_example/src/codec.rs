use actix::Message;
use byteorder::BigEndian;
use bytes::BytesMut;
use serde_derive::{Deserialize, Serialize};
use serde_json as json;
use tokio_util::codec::{Decoder, Encoder};

/// Client request
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatRequest {
    List,
    Join(String),
    Message(String),
    Ping,
}

/// Client response
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatResponse {
    /// list of rooms
    Rooms(Vec<String>),
    /// room joined
    Joined(String),
    Message(String),
    Ping,
}

/// codec used by the Server
pub struct ServerChatCodec;

/// Decode bytes into a ChatRequest
impl Decoder for ServerChatCodec {
    type Item = ChatRequest;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };
        if src.len() >= size + 2 {
            src.advance(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatRequest>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

/// Encode a ChatRequest into bytes
impl Encoder for ServerChatCodec {
    type Item = ChatResponse;
    type Error = std::io::Error;

    fn encode(&mut self, msg: ChatResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();
        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}

/// codec used by the Client
pub struct ClientChatCodec;

/// Decode bytes into a ChatResponse
impl Decoder for ClientChatCodec {
    type Item = ChatResponse;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };
        if src.len() >= size + 2 {
            src.advance(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

/// Encode a ChatRequest into bytes
impl Encoder for ClientChatCodec {
    type Item = ChatRequest;
    type Error = std::io::Error;

    fn encode(&mut self, msg: ChatRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();
        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}
