use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    response::Response,
};
use deku::DekuContainerWrite;
use tokio::{select, sync::broadcast::Receiver};

use crate::{
    model::{Role, VoteEvent, ROOMS},
    utils::jwt,
};

use super::room_id::RoomID;

pub async fn handle_request(
    ws: WebSocketUpgrade,
    // Waiting to see if custom headers will be supported
    // on the WebSocket protocol
    // see: https://github.com/whatwg/websockets/issues/16
    //
    // user: User,
    Path(room_id): Path<RoomID>,
) -> Response {
    let rooms = ROOMS.read().unwrap();
    let room = rooms.iter().find(|r| r.id == room_id).unwrap();
    let receiver = room.channel.subscribe();
    drop(rooms);

    ws.on_upgrade(move |socket| handle_room_websocket(socket, receiver))
}

async fn handle_room_websocket(mut socket: WebSocket, mut room_receiver: Receiver<VoteEvent>) {
    // Check the first message is an admin auth token
    match socket.recv().await {
        Some(Ok(Message::Text(token))) => {
            let Ok(user) = jwt::verify(token.trim()) else {
                log::warn!("Received invalid auth token: {}", token);
                return;
            };

            if user.role != Role::Admin {
                log::warn!("Received non-admin auth: {} (uid)", user.uid);
                return;
            }

            log::warn!("Admin connected to room");
        }
        Some(Err(e)) => {
            eprintln!("Error receiving websocket auth token: {}", e);
            return;
        }
        None => {
            log::warn!("Socket closed unexpectedly before receiving auth message");
            return;
        }
        Some(Ok(msg)) => {
            log::warn!("Received invalid auth message: {:?}", msg);
            return;
        }
    }

    loop {
        select! {
            msg = socket.recv() => {
                if matches!(msg, Some(Ok(Message::Close(_))) | None | Some(Err(_))) {
                    log::info!("Admin disconnected from room");
                    return;
                } else {
                    log::warn!("Received invalid websocket message: {:?}", msg);
                }
            }
            vote = room_receiver.recv() => {
                let Ok(vote) = vote else {
                    return;
                };
                let encoded = vote.to_bytes().unwrap();
                if let Err(e) = socket.send(Message::Binary(encoded)).await {
                    eprintln!("Error sending vote: {}", e);
                    break;
                }
            }
        }
    }
}