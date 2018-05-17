use websocket;
use websocket::OwnedMessage;
use websocket::sync::Server;
use websocket::server::upgrade::WsUpgrade;
use serde_json;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::io::Error as StdError;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::time::Duration;
use std::net::TcpStream;
use websocket::server::upgrade::sync::Buffer;

use dsc_manager::{DSCManagerMutex, UpdateManager};
use session::Update as SessionUpdate;
use web::types::{Config, RequestType, SendType, ClientSenders};



/// Start websocket server on given address and port
///
/// config:     Websocket config
/// manager:    DSC Manager to use
pub fn start_websocket<'a>(config: Config, manager: DSCManagerMutex) -> Result<(), StdError> {
    println!("[web::socket][start_websocket] start on: {}", config.address_port);
    let server = Server::bind(config.address_port)?;
    let client_senders: ClientSenders = Arc::new(Mutex::new(vec![]));

    // dispatcher thread
    start_broadcast_thread(client_senders.clone(), manager.clone());

    // client threads
    for request in server.filter_map(Result::ok) {
        let (client_tx, client_rx) = mpsc::channel();
        client_senders.lock().unwrap().push(client_tx);
        connect_client(client_rx, request, manager.clone());
    }
    Ok(())
}



type ClientRequest = WsUpgrade<TcpStream, Option<Buffer>>;
/// Spawn thread(s) for this client connection. Check if the websocket has the correct connection
/// protocol string, otherwise close the connection.
/// We send initialy Session and Config messages to the client.
///
/// One additional thread will check for new messages from the socket, and post them into an
/// internal channel. The main client thread will listen to this channel, as well as the public_rx
/// channel an process the messages.
///
/// public_rx:  Channel for messages which should be send to the client, e.g. for a broadcast
///             the tx channel is in the client_senders array
/// request:    Client connection request
/// manager:    DSCManagerMutex to perform requested actions
fn connect_client(public_rx: mpsc::Receiver<String>, request: ClientRequest, manager: DSCManagerMutex) {
    // Spawn a new thread for each connection.
    thread::spawn(move || {
        if !request.protocols().contains(&"rust-websocket".to_string()) {
            let _ = request.reject();
            return;
        }
        let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        let ip = client.peer_addr().unwrap_or(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
        );
        println!("Connection from {}", ip);

        // Send current session on connect
        let text = serde_json::to_string(&manager.lock().unwrap().session).unwrap();
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
                    process_message(&manager, message, &mut sender);
                }

                // Send messages we got from client_senders
                if let Ok(message) = public_rx.try_recv() {
                    let message = OwnedMessage::Text(message);
                    sender.send_message(&message).unwrap_or(());
                }

                thread::sleep(Duration::from_millis(100));
            }
        }
    });
}



type WSSender = websocket::sender::Writer<TcpStream>;
/// Process a given message, wich was receved from any client.
///
/// manager:    DSCMangerMutex to perform actions
/// message:    Message to parse
/// sender:     sender reference to directly send messages back to the client
fn process_message(manager: &DSCManagerMutex, message: OwnedMessage, sender: &mut WSSender) {
    match message {
        OwnedMessage::Close(_) => {
            let message = OwnedMessage::Close(None);
            sender.send_message(&message).unwrap_or(());
            // This client will be remove from the client_senders list by
            // the next broadcast call
            return;
        },
        OwnedMessage::Ping(ping) => {
            let message = OwnedMessage::Pong(ping);
            sender.send_message(&message).unwrap_or(());
        },
        OwnedMessage::Text(text) => {
            match serde_json::from_str(&text) {
                Ok(request_type) => {
                    println!("{:?}", request_type);
                    match request_type {
                        RequestType::NewTarget => {
                            println!("RequestType::NewTarget");
                            manager.lock().unwrap().new_target(false);
                        },
                        RequestType::SetDisciplin{ name } => {
                            manager.lock().unwrap().set_disciplin_by_name(&name);
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
