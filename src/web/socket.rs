use serde_json;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::io::Error as StdError;
use std::time::Duration;
use std::net::TcpStream;
use websocket::OwnedMessage;
use websocket::sync::Server;
use websocket::server::upgrade::WsUpgrade;
use websocket::server::upgrade::sync::Buffer;
use std::process::Command;

use dsc_manager::{DSCManagerMutex, UpdateManager};
use session::Update as SessionUpdate;
use super::{Config, RequestType, SendType, ClientSenders};
use config::Config as DSCConfig;


/// Start websocket server on given address and port
///
/// config:     Websocket config
/// manager:    DSC Manager to use
pub fn start_websocket<'a>(socket_config: Config, config: DSCConfig, manager: DSCManagerMutex) -> Result<(), StdError> {
    println!("[web::socket][start_websocket] start on: {}", socket_config.address_port);
    let server = Server::bind(socket_config.address_port)?;
    let client_senders: ClientSenders = Arc::new(Mutex::new(vec![]));

    // dispatcher thread
    start_broadcast_thread(client_senders.clone(), manager.clone());

    // client threads
    for request in server.filter_map(Result::ok) {
        let (client_tx, client_rx) = mpsc::channel();
        client_senders.lock().unwrap().push(client_tx);
        connect_client(client_rx, request, config.clone(), manager.clone());
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
fn connect_client(public_rx: mpsc::Receiver<String>, request: ClientRequest, config: DSCConfig, manager: DSCManagerMutex) {
    // Spawn a new thread for each connection.
    thread::spawn(move || {
        // if !request.protocols().contains(&"rust-websocket".to_string()) {
        //     let _ = request.reject();
        //     return;
        // }
        // let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        let mut client = request.accept().unwrap();
        

        {   // Send the config on connect
            let config_msg = SendType::Config {
                config: config,
            };
            let text = serde_json::to_string(&config_msg).unwrap();
            let message = OwnedMessage::Text(text);
            client.send_message(&message).unwrap_or(());
        }
            
        {   // Send current session on connect
            let session_msg = SendType::Session {
                session: manager.lock().unwrap().session.clone(),
            };
            let text = serde_json::to_string(&session_msg).unwrap();
            let message = OwnedMessage::Text(text);
            client.send_message(&message).unwrap_or(());
        }

        if let Ok((mut receiver, mut sender)) = client.split() {
            // Spawn custom thread for reading incoming_message from the client
            // all messages are forwarded to the rx channel
            let(tx, rx) = mpsc::channel::<OwnedMessage>();
            thread::spawn(move || {
                for incoming_message in receiver.incoming_messages() {
                    if let Ok(message) = incoming_message {
                        
                        // Check if the message is a close message, if so we save this in a flag
                        let mut exit_after = false;
                        match message {
                            OwnedMessage::Close(_) => exit_after = true,
                            _ => {},
                        }
                        
                        // Send the message to the main client thread
                        tx.send(message).unwrap();
                        
                        // if we receved a close message, break the loop and exit this thread
                        if exit_after {
                            break;
                        }
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
                            // This client will be remove from the client_senders list by
                            // the next broadcast call, we return here, to exit the loop end the
                            // thread
                            break;
                        },
                        OwnedMessage::Ping(ping) => {
                            println!("ping");
                            let message = OwnedMessage::Pong(ping);
                            sender.send_message(&message).unwrap_or(());
                        },
                        OwnedMessage::Text(text) => process_message(&manager, text),
                        _ => {},
                    }

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



// type WSSender = websocket::sender::Writer<TcpStream>;

/// Process a given message, wich was receved from any client.
///
/// manager:    DSCMangerMutex to perform actions
/// message:    String to parse
/// sender:     sender reference to directly send messages back to the client
fn process_message(manager: &DSCManagerMutex, message: String) {
    match serde_json::from_str(&message) {
        Ok(request_type) => {
            println!("{:?}", request_type);
            match request_type {
                RequestType::NewTarget => {
                    println!("RequestType::NewTarget");
                    manager.lock().unwrap().new_target();
                },
                RequestType::SetDisciplin{ name } => {
                    manager.lock().unwrap().set_disciplin_by_name(&name);
                },
                RequestType::SetPart{ name, force_new_part } => {
                    manager.lock().unwrap().set_part(name, force_new_part);
                },
                RequestType::Print => {
                    manager.lock().unwrap().print_session();
                },
                RequestType::Shutdown => {
                    // TODO path
                    let _ = Command::new("sudo").arg("/sbin/shutdown");
                },
                RequestType::DisablePaperAck => {
                    manager.lock().unwrap().disable_paper_ack();
                }
                RequestType::CheckPaper => {
                    manager.lock().unwrap().check_paper();
                }
            };
        },
        Err(err) => println!("Parsing Error {:?}", err),
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
