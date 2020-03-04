// //! `ChatServer` is an actor. It maintains list of connection client session.
// //! And manages available rooms. Peers send messages to other peers in same
// //! room through `ChatServer`.
// use crate::session;
// use crate::session::ChatSession;
// use actix::prelude::*;
// use std::collections::{HashMap, HashSet};
//
// /// New chat session is established
// #[derive(Message)]
// #[rtype(result = "usize")]
// pub struct Connect {
//     pub addr: Addr<session::ChatSession>,
// }
//
// /// chat session is disconnected
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: usize,
// }
//
// /// Send message to room
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Message {
//     /// id of the client session
//     pub id: usize,
//     pub msg: String,
//     pub room: String,
// }
//
// /// List of available rooms
// pub struct ListRooms;
//
// impl actix::Message for ListRooms {
//     type Result = Vec<String>;
// }
//
// /// Join a room. If the room doesn't exist, create a new one.
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Join {
//     /// client's session id
//     pub id: usize,
//     /// room name
//     pub name: String,
// }
//
// pub struct ChatServer {
//     // map session id to a ChatSession actor address
//     sessions: HashMap<usize, Addr<ChatSession>>,
//     // map room to a set of sessions ids
//     rooms: HashMap<String, HashSet<usize>>,
// }
//
// impl Default for ChatServer {
//     fn default() -> ChatServer {
//         // default room
//         let mut rooms = HashMap::new();
//         rooms.insert("Main".to_string(), HashSet::new());
//
//         ChatServer {
//             rooms,
//             sessions: HashMap::new(),
//         }
//     }
// }
//
// impl ChatServer {
//     // send a message to all the users in a room
//     fn send_message(&self, room: &str, message: &str, skip_id: usize) {
//         if let Some(sessions) = self.rooms.get(room) {
//             for id in sessions {
//                 if *id != skip_id {
//                     if let Some(addr) = self.sessions.get(id) {
//                         addr.do_send(session::Message(message.to_string()));
//                     }
//                 }
//             }
//         }
//     }
//
//     /// Remove a session and return all the rooms the session was in.
//     fn remove_session(&mut self, id: usize) -> Vec<String> {
//         let mut rooms: Vec<String> = vec![];
//         // remove address
//         if self.sessions.remove(&id).is_some() {
//             // remove session from all rooms
//             for (name, sessions) in &mut self.rooms {
//                 if sessions.remove(&id) {
//                     rooms.push(name.to_string());
//                 }
//             }
//         }
//
//         rooms
//     }
// }
//
// impl Actor for ChatServer {
//     type Context = Context<Self>;
// }
//
// /// Handle Connect messages. Register new session and assign a unique id to this session
// impl Handler<Connect> for ChatServer {
//     type Result = usize;
//
//     fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
//         println!("Someone joined");
//
//         // notify all the users in the same room
//         self.send_message("Main", "Someone joined", 0);
//
//         // register session with random id
//         let id = rand::thread_rng().gen::<usize>();
//         self.sessions.insert(id, msg.addr);
//
//         // auto join session to the Main room
//         self.rooms.get_mut("Main").unwrap().insert(id);
//
//         id
//     }
// }
//
// /// Handle Disconnect messages.
// impl Handler<Disconnect> for ChatServer {
//     type Result = ();
//
//     fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) {
//         println!("Someone disconnected");
//
//         // send message to other users
//         for room in self.remove_session(msg.id) {
//             self.send_message(&room, "Someone disconnected", 0);
//         }
//     }
// }
//
// impl Handler<Message> for ChatServer {
//     type Result = ();
//
//     fn handle(&mut self, msg: Message, _ctx: &mut Context<Self>) {
//         self.send_message(&msg.room, msg.msg.as_ref(), msg.id);
//     }
// }
//
// impl Handler<ListRooms> for ChatServer {
//     type Result = MessageResult<ListRooms>;
//
//     fn handle(&mut self, msg: ListRooms, _ctx: &mut Context<Self>) -> Self::Result {
//         let rooms = self.rooms.keys().iter().map(|room| room.to_owned());
//         MessageResult(rooms)
//     }
// }
//
// impl Handler<Join> for ChatServer {
//     type Result = ();
//
//     // TODO: finish this
//     fn handle(&mut self, msg: Join, _ctx: &mut Context<Self>) {}
// }
