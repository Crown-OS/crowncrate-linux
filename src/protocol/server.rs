
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Result},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc::{Sender}},
    thread,
};

use crate::Message;


pub struct Client {
    reader: BufReader<TcpStream>
}

pub struct Server {
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>
}

impl Server {
    pub fn create() -> Self {
        Server { clients: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn listen(&mut self, port: i16, tx: Sender<Message>) -> Result<()> {

        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

        for stream in listener.incoming() {
            let stream = stream?;
            let clients = self.clients.clone();
            let peer = stream.peer_addr()?;
            let tx = tx.clone();

            thread::spawn(move || {
                {
                    clients.lock();
                    let mut clients = clients.lock().unwrap();

                    clients.insert(peer, Client {
                        reader: BufReader::new(stream)
                    });
                }

                let mut string = String::new();

                {
                    let mut clients = clients.lock().unwrap();
                    let client = clients.get_mut(&peer).unwrap();
                    let _ = client.reader.read_line(&mut string);
                    tx.send(Message {
                        task: string,
                        body: "TEst Body".to_string()
                    })
                }
            });
        }
        Ok(())
    }

    fn parse_tcp
}
