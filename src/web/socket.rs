use websocket::OwnedMessage;
use websocket::sync::Server;
use serde_json::{self, Error};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::time::Duration;

use dsc_manager::{DSCManagerMutex, UpdateManager};
use session::Update as SessionUpdate;
use web::types::{Config, RequestType, SendType, ClientSenders};



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
    start_broadcast_thread(client_senders.clone(), manager.clone());

    // client threads
    for request in server.filter_map(Result::ok) {
        let (client_tx, client_rx) = mpsc::channel();
        client_senders.lock().unwrap().push(client_tx);

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
            let text = serde_json::to_string(&manager_clone.lock().unwrap().session).unwrap();
            let message = OwnedMessage::Text(text);
            match client.send_message(&message) {
                Ok(_) => {},
                Err(err) => println!("{:?}", err),
            };



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
                                // This client will be remove from the client_senders list by
                                // the next broadcast call
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
                                                manager_clone.lock().unwrap().new_target(false);
                                            },
                                            RequestType::SetDisciplin{ name } => {
                                                manager_clone.lock().unwrap().set_disciplin_by_name(&name);
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
}




/// Creates a Update channel and sets it in the dscmanager. Then we start a thread which checks
/// this channel for update and sends them to each connected socket client.
/// client_senders:     List of clients
/// manager:            DSCManager, to set update channel
fn start_broadcast_thread(client_senders: ClientSenders, manager: DSCManagerMutex) {
    let (on_update_tx, on_update_rx) = mpsc::channel::<SendType>();
    manager.lock().unwrap().on_update_tx = Some(on_update_tx);
    thread::spawn(move || {
        loop {
            if let Ok(msg) = on_update_rx.try_recv() {
                match serde_json::to_string(&msg) {
                    Ok(text) => broadcast_to_all(client_senders.clone(), text),
                    Err(err) => println!("{}", err),
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

/// Send given message to all active client. If a client is closed, we clean up the acitve clients
/// list by removing them.
fn broadcast_to_all(client_senders: ClientSenders, message: String) {
    // We save the index of closed sockets here
    let mut to_remove: Vec<usize> = Vec::new();
    let mut senders = client_senders.lock().unwrap();
    for (index, sender) in senders.iter().enumerate() {
        match sender.send(message.clone()) {
            Result::Ok(_) => {},
            Result::Err(_) => to_remove.push(index),
        };
    }

    // Remove all closed sockets we detected
    // by reverse looping over the to_remove array
    for index in to_remove.iter().rev() {
        senders.remove(*index);
    }
}







fn parse_request(text: &str) -> Result<RequestType, Error> {
    let request_type: RequestType = serde_json::from_str(text)?;
    return Ok(request_type);
}
