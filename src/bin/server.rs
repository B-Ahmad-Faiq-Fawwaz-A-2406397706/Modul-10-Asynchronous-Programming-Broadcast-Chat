use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio::sync::Mutex;
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

type Users = Arc<Mutex<HashMap<SocketAddr, String>>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Register,
    Users,
    Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_array: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    from: String,
    message: String,
    time: u128,
}

async fn users_message(users: &Users) -> Result<String, serde_json::Error> {
    let names = users.lock().await.values().cloned().collect();
    serde_json::to_string(&WebSocketMessage {
        message_type: MessageType::Users,
        data_array: Some(names),
        data: None,
    })
}

fn chat_message(from: String, message: String) -> Result<String, serde_json::Error> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();

    let data = serde_json::to_string(&ChatMessage {
        from,
        message,
        time,
    })?;

    serde_json::to_string(&WebSocketMessage {
        message_type: MessageType::Message,
        data_array: None,
        data: Some(data),
    })
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Users,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            let parsed = serde_json::from_str::<WebSocketMessage>(text);

                            match parsed {
                                Ok(WebSocketMessage { message_type: MessageType::Register, data: Some(username), .. }) => {
                                    users.lock().await.insert(addr, username.clone());
                                    println!("Registered {addr:?} as {username}");
                                    bcast_tx.send(users_message(&users).await?)?;
                                }
                                Ok(WebSocketMessage { message_type: MessageType::Message, data: Some(message), .. }) => {
                                    let sender = users
                                        .lock()
                                        .await
                                        .get(&addr)
                                        .cloned()
                                        .unwrap_or_else(|| addr.to_string());
                                    println!("From {sender}: {message}");
                                    bcast_tx.send(chat_message(sender, message)?)?;
                                }
                                Ok(_) => {}
                                Err(error) => {
                                    println!("Invalid message from {addr:?}: {error}");
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }

    let removed = users.lock().await.remove(&addr).is_some();
    if removed {
        bcast_tx.send(users_message(&users).await?)?;
    }
    println!("Disconnected {addr:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(32);
    let users = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();

        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}
