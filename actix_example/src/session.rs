use crate::codec::{ChatResponse, ServerChatCodec};
use crate::server::{ChatServer, Connect, Disconnect};
use actix::clock::{Duration, Instant};
use actix::io::FramedWrite;
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Context};
use futures::io::WriteHalf;
use std::net::TcpStream;

// ChatServer sends this message to sessions
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

// Actor that's responsible for tcp peer communications
pub struct ChatSession {
    /// unique session id
    id: usize,
    /// this is the address of the chat server
    addr: Addr<ChatServer>,
    /// Client must send heartbeat pings at least once every 10 seconds, otherwise we drop the connection
    heartbeat: Instant,
    /// joined room
    room: String,
    framed: FramedWrite<WriteHalf<TcpStream>, ServerChatCodec>,
}

impl Actor for ChatSession {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // start the heartbeat process on session start
        self.heartbeat(ctx);

        // Register self in chat server.
        self.addr
            .send(Connect {
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(move |res, act, _| {
                act.id = res.unwrap();
                async {}.into_actor(act)
            })
            .wait(ctx);
    }
}

impl ChatSession {
    fn heartbeat(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.heartbeat) > Duration::new(10, 0) {
                // heartbeat timed out
                println!("Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect { id: act.id });

                // stop actor
                ctx.stop();
            }
            act.framed.write(ChatResponse::Ping);
            act.heatbeat(ctx);
        });
    }
}
