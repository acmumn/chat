use std::io;

use bytes::BytesMut;
use irc::{
    error::IrcError,
    proto::{Command, IrcCodec, Message},
};
use tokio_io::codec::{Decoder, Encoder};

#[derive(Debug, Message)]
pub enum Request {
    Command(Command),
}

// Client -> Server
pub struct ClientCodec {
    inner: IrcCodec,
}

impl ClientCodec {
    pub fn new() -> Self {
        ClientCodec {
            inner: IrcCodec::new("utf-8").unwrap(),
        }
    }
}

impl Decoder for ClientCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(buf).map_err(|err| match err {
            IrcError::Io(inner) => inner,
            _ => unreachable!(),
        })? {
            Some(msg) => Ok(Some(Request::Command(msg.command))),
            _ => Ok(None),
        }
    }
}

impl Encoder for ClientCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        self.inner
            .encode(
                match item {
                    _ => Message::new(None, "PING", vec![], None).unwrap(),
                },
                buf,
            ).map_err(|err| match err {
                IrcError::Io(inner) => inner,
                _ => unreachable!(),
            })
    }
}
