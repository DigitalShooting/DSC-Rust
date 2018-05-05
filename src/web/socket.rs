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


type ClientSenders = Arc<Mutex<Vec<mpsc::Sender<String>>>>;

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



pub fn start_websocket<'a>(config: Config, manager: DSCManagerMutex) {
    println!("[web::socket][start_websocket] start on: {}", config.address_port);

    let server;
    match Server::bind(config.address_port) {
        Ok(s) => server = s,
        Err(err) => {
            println!("{:?}", err);
            return;
        },
    }

    let client_senders: ClientSenders = Arc::new(Mutex::new(vec![]));



    // dispatcher thread
    start_bradcast_thread(client_senders.clone(), manager.clone());





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
        let manager_clone = manager.clone();
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



            // Send current session on connect
            match manager_clone.lock() {
                Ok(mut manager) => {
                    let text = serde_json::to_string(&manager.session).unwrap();
                    let message = OwnedMessage::Text(text);
                    match client.send_message(&message) {
                        Ok(_) => {},
                        Err(err) => println!("{:?}", err),
                    };
                }
                Err(err) => print!("{:?}", err),
            }



            if let Ok((mut receiver, mut sender)) = client.split() {

                // Spawn custom thread for reading incoming_message from the client
                // all messages are forwarded to the rx channel
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

                                                match manager_clone.lock() {
                                                    Ok(mut manager) => manager.new_target(),
                                                    Err(err) => print!("{:?}", err),
                                                }
                                            },
                                            RequestType::SetDisciplin{ name } => {

                                                // TODO get disziplin by name
                                                let discipline = helper::dsc_demo::lg_discipline();

                                                match manager_clone.lock() {
                                                    Ok(mut manager) => manager.set_disciplin(discipline),
                                                    Err(err) => print!("{:?}", err),
                                                }
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

                    // Send messages we got from client_senders
                    if let Ok(message) = client_rx.try_recv() {
                        let message = OwnedMessage::Text(message);
                        sender.send_message(&message).unwrap_or(());
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }
    println!("end socker");
}





fn start_bradcast_thread(client_senders: ClientSenders, manager: DSCManagerMutex) {
    match manager.lock() {
        Ok(mut manager) => {
            let (on_update_tx, on_update_rx) = mpsc::channel::<Update>();
            manager.on_update_tx = Some(on_update_tx);
            thread::spawn(move || {
                loop {
                    if let Ok(msg) = on_update_rx.try_recv() {
                        match msg {
                            Update::Data(string) => {
                                match client_senders.lock() {
                                    Ok(senders) => {
                                        for sender in senders.iter() {
                                            match sender.send(string.clone()) {
                                                Result::Ok(_) => {},
                                                Result::Err(err) => {
                                                    println!("send to client: {}", err);
                                                    // TODO clean up closed senders
                                                    continue;
                                                },
                                            };
                                        }
                                    },
                                    Err(err) => println!("client_senders lock: {}", err),
                                }
                            },
                            Update::Error(err) => println!("{}", err),
                        }
                    }
                    thread::sleep(Duration::from_millis(500));
                }
            });
        }
        Err(err) => println!("{}", err),
    }
}







fn parse_request(text: &str) -> Result<RequestType, Error> {
    let request_type: RequestType = serde_json::from_str(text)?;
    return Ok(request_type);
}
