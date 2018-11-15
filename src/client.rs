use std::io;

use actix::{
    io::{FramedWrite, WriteHandler},
    Actor, Addr, Context, StreamHandler,
};
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;

use codec::{ClientCodec, Request};
use server::Server;

pub struct Client {
    session: Addr<Session>,
}

pub struct Session {
    server: Addr<Server>,
}

impl Session {
    pub fn new(
        server: Addr<Server>,
        framed: FramedWrite<WriteHalf<TcpStream>, ClientCodec>,
    ) -> Session {
        Session { server }
    }
}

impl WriteHandler<io::Error> for Session {}

impl StreamHandler<Request, io::Error> for Session {
    fn handle(&mut self, msg: Request, ctx: &mut Self::Context) {
        match msg {
            Request::Command(cmd) => println!("command: {:?}", cmd),
        }
    }
}

impl Actor for Session {
    type Context = Context<Self>;
}
