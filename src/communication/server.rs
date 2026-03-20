use std::{
    collections::HashMap,
    io::{BufReader, Result},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    actions::{action_manager::ActionManager}, communication::Message
};

pub struct Client {
    reader: BufReader<TcpStream>,
}

pub struct Server {
    pub clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    action_manager: ActionManager
}

impl Server {
    pub fn create(actions_manager: ActionManager) -> Self {
        Server {
            clients: Arc::new(Mutex::new(HashMap::default())),
            action_manager: actions_manager
        }
    }

    pub fn listen(&mut self, port: i16) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

        for stream in listener.incoming() {
            let stream = stream?;
            let clients = self.clients.clone();
            let peer = stream.peer_addr()?;

            {
                let stream_clone = stream.try_clone().unwrap();
                let reader = BufReader::new(stream_clone);
                let mut clients = clients.lock().unwrap();
                
                if let None = clients.get(&peer) {
                    clients.insert(peer, Client { reader: reader });
                }
            }

            thread::spawn(move || {
                let reader = BufReader::new(stream);
                let deserializer = serde_cbor::Deserializer::from_reader(reader);
                let message_stream = deserializer.into_iter::<Message>();

                for message in message_stream {
                    match message {
                        Ok(message) => {
                            self.action_manager.notify(message);
                        }

                        Err(e) => {
                            println!("Client {} disconnected or error: {:?}", peer.ip(), e);
                            break;
                        }
                    }
                }
            });
        }
        Ok(())
    }
}
