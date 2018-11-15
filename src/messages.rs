use actix::{Addr, Message};

use Session;

pub struct ClientConnect {
    addr: Addr<Session>,
}

impl Message for ClientConnect {
    type Result = u32;
}
