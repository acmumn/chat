#[macro_use]
extern crate actix;
extern crate actix_net;
extern crate bytes;
extern crate futures;
extern crate irc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_tcp;

mod client;
mod codec;
pub mod messages;
mod server;
mod user;

use std::net::SocketAddr;
use std::str::FromStr;

use actix::{io::FramedWrite, Actor, Addr, AsyncContext, Context, Handler, StreamHandler};
use futures::Stream;
use tokio_codec::FramedRead;
use tokio_io::AsyncRead;
use tokio_tcp::{TcpListener, TcpStream};

use client::Session;
use codec::ClientCodec;

struct ServerWrapper {
    srv: Addr<server::Server>,
}

impl Actor for ServerWrapper {
    type Context = Context<Self>;
}

#[derive(Message)]
pub struct TcpConnect(pub TcpStream, pub SocketAddr);

impl Handler<TcpConnect> for ServerWrapper {
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _: &mut Context<Self>) -> Self::Result {
        let server = self.srv.clone();
        Session::create(move |ctx| {
            let (r, w) = msg.0.split();
            Session::add_stream(FramedRead::new(r, ClientCodec::new()), ctx);
            Session::new(server, FramedWrite::new(w, ClientCodec::new(), ctx))
        });
    }
}

pub fn run() {
    let srv = server::Server::default().start();

    let addr = SocketAddr::from_str("127.0.0.1:12345").unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    ServerWrapper::create(|ctx| {
        ctx.add_message_stream(listener.incoming().map_err(|_| ()).map(|stream| {
            let addr = stream.peer_addr().unwrap();
            TcpConnect(stream, addr)
        }));
        ServerWrapper { srv }
    });
}
