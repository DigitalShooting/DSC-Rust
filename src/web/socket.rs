use websocket;
use websocket::OwnedMessage;
use websocket::sync::Server;

use serde_json;
use serde_json::Error;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::time::Duration;

use dsc_manager::*;
use device_api::api::{Action};

use helper;



pub struct Config {
    pub address_port: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RequestType {
    NewTarget,
    SetDisciplin { name: String },
    Shutdown,
}



pub fn start_websocket<'a>(config: Config, set_event_tx: mpsc::Sender<Event>, on_update_rx: mpsc::Receiver<Update>) {
    println!("[web::socket][start_websocket] start on: {}", config.address_port);

    let server;
    match Server::bind(config.address_port) {
        Ok(s) => server = s,
        Err(err) => {
            println!("{:?}", err);
            return;
        },
    }

    let client_senders: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(vec![]));

    // dispatcher thread
    {
        let client_senders = client_senders.clone();
        thread::spawn(move || {
            while let Ok(msg) = on_update_rx.recv() {
                match msg {
                    Update::Data(string) => {
                        match client_senders.lock() {
                            Ok(senders) => {
                                for sender in senders.iter() {
                                    match sender.send(string.clone()) {
                                        Result::Ok(_) => {},
                                        Result::Err(err) => {
                                            println!("{:?}", err);
                                            return;
                                        },
                                    };
                                }
                            },
                            Err(err) => {
                                println!("{:?}", err);
                                return;
                            },
                        }
                    },
                    Update::Error(err) => println!("{:?}", err),
                }
            }
        });
    }

    // client threads
    for request in server.filter_map(Result::ok) {
        let (client_tx, client_rx) = mpsc::channel();

        match client_senders.lock() {
            Ok(mut senders) => senders.push(client_tx),
            Err(err) => {
                println!("{:?}", err);
                return;
            },
        };

        // Spawn a new thread for each connection.
        let set_event_tx_copy = set_event_tx.clone();
        thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                match request.reject() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("{:?}", err);
                    },
                };
                return;
            }

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap_or(
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
            );
            println!("Connection from {}", ip);


            let message = OwnedMessage::Text("SERVER: Connected.".to_string());
            match client.send_message(&message) {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                },
            };

            if let Ok((mut receiver, mut sender)) = client.split() {
                let(tx, rx) = mpsc::channel::<OwnedMessage>();
                thread::spawn(move || {
                    for incoming_message in receiver.incoming_messages() {
                        if let Ok(message) = incoming_message {
                            tx.send(message).unwrap();
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                });

                loop {
                    if let Ok(message) = rx.try_recv() {
                        match message {
                            OwnedMessage::Close(_) => {
                                let message = OwnedMessage::Close(None);
                                sender.send_message(&message).unwrap_or(());
                                println!("Client {} disconnected", ip);
                                return;
                            },
                            OwnedMessage::Ping(ping) => {
                                let message = OwnedMessage::Pong(ping);
                                sender.send_message(&message).unwrap_or(());
                            },
                            OwnedMessage::Text(text) => {
                                // dispatcher.send(text).unwrap_or(());


                                match parse_request(&text) {
                                    Ok(request_type) => {
                                        println!("{:?}", request_type);
                                        match request_type {
                                            RequestType::NewTarget => {
                                                println!("RequestType::NewTarget");
                                                let _ = set_event_tx_copy.send(Event::NewTarget);
                                            },
                                            RequestType::SetDisciplin{ name } => {

                                                // TODO get disziplin by name
                                                let discipline = helper::dsc_demo::lg_discipline();

                                                let _ = set_event_tx_copy.send(Event::SetDisciplin(discipline));
                                            },
                                            RequestType::Shutdown => {
                                                println!("Not Implemented");
                                            },
                                        };

                                    },
                                    Err(err) => println!("Parsing Error {:?}", err),
                                }
                            },
                            _ => {},
                        }
                    }
                    if let Ok(message) = client_rx.try_recv() {
                        let message = OwnedMessage::Text(message);
                        sender.send_message(&message).unwrap_or(());
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }
}






fn parse_request(text: &str) -> Result<RequestType, Error> {
    let request_type: RequestType = serde_json::from_str(text)?;
    return Ok(request_type);
}
