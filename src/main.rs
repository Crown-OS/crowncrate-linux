mod clipboard;
mod communication;
mod config;
mod discovery;
mod logging;
mod services;

use std::{sync::mpsc, thread};

use communication::{Message, Server};

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();
    thread::spawn(move || {
        let mut server = Server::create();
        server.listen(5252, tx).ok();
    });

    while let Ok(message) = rx.recv() {
        println!("{:?}", message);
    }
}
