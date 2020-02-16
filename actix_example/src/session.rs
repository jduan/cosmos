use crate::codec::{ChatResponse, ServerChatCodec};
use crate::message::Disconnect;
use actix::clock::{Duration, Instant};
use actix::io::FramedWrite;
use actix::{Actor, Addr, AsyncContext, Context};
use futures::io::WriteHalf;
use std::net::TcpStream;

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
