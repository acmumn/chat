extern crate actix;
extern crate chat;

use actix::System;

fn main() {
    let sys = System::run(|| {
        chat::run();
    });
}
