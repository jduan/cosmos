use crate::session;
/// Messages for chat server communication
use actix::prelude::*;

/// New chat session is established
#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Addr<session::ChatSession>,
}

/// chat session is disconnected
#[derive(Message)]
#[rtype(result = "usize")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to room
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message {
    /// id of the client session
    pub id: usize,
    pub msg: String,
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join a room. If the room doesn't exist, create a new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// client id
    pub id: usize,
    /// room name
    pub name: String,
}
