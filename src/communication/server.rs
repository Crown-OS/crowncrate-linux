use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Result},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

use crate::{
    communication::{Message, Method},
    services::{service::Service, shutdown::ShutdownService, volume::VolumeService},
};

pub struct Client {
    reader: BufReader<TcpStream>,
}

pub struct Server {
    pub clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
}

impl Server {
    pub fn create() -> Self {
        Server {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn listen(&mut self, port: i16, tx: Sender<Message>) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

        for stream in listener.incoming() {
            let stream = stream?;
            let clients = self.clients.clone();
            let peer = stream.peer_addr()?;
            let tx = tx.clone();

            {
                let stream_clone = stream.try_clone().unwrap();
                let reader = BufReader::new(stream_clone);
                let mut clients = clients.lock().unwrap();

                println!("{}", peer.ip());

                clients.insert(peer, Client { reader: reader });
            }

            thread::spawn(move || {
                let reader = BufReader::new(stream);
                let deserializer = serde_cbor::Deserializer::from_reader(reader);
                let message_stream = deserializer.into_iter::<Message>();

                for message_result in message_stream {
                    match message_result {
                        Ok(message) => match message.method {
                            Method::VOLUME => {
                                VolumeService::handle_message(message);
                            }
                            Method::SHUTDOWN => {
                                ShutdownService::handle_message(message);
                            }
                            _ => {}
                        },

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
