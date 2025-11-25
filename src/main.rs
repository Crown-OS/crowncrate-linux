mod protocol;
mod logging;

use std::{sync::mpsc, thread};

use protocol::{Server};

pub struct Message {
    task: String,
    body: String
}

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();
    thread::spawn(move || {
        let mut server = Server::create();
        server.listen(3001, tx);
    });

    while let Ok(message) = rx.recv() {
        println!("task: {}", message.task);
        println!("body: {}", message.body);
    }
    
}
