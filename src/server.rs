use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use messages;
use Session;
use TcpConnect;

pub struct Server {
    clients: HashMap<usize, Addr<Session>>,
    counter: u32,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            clients: HashMap::new(),
            counter: 0,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<messages::ClientConnect> for Server {
    type Result = u32;
    fn handle(&mut self, msg: messages::ClientConnect, _: &mut Context<Self>) -> Self::Result {
        println!("shiet");
        0
    }
}
