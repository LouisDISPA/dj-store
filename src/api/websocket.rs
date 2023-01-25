use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
};
use deku::DekuContainerWrite;
use reqwest::StatusCode;
use tokio::sync::broadcast::Receiver;

use crate::model::{Role, User, VoteEvent, ROOMS};

use super::room_id::RoomID;

pub async fn handle_request(
    ws: WebSocketUpgrade,
    user: User,
    Path(room_id): Path<RoomID>,
) -> Response {
    if user.role != Role::Admin {
        return (StatusCode::FORBIDDEN, "You are not an admin").into_response();
    }
    let rooms = ROOMS.read().unwrap();
    let room = rooms.iter().find(|r| r.id == room_id).unwrap();
    let receiver = room.channel.subscribe();
    drop(rooms);

    ws.on_upgrade(move |socket| handle_room_websocket(socket, receiver))
}

async fn handle_room_websocket(mut socket: WebSocket, mut room_receiver: Receiver<VoteEvent>) {
    while let Ok(vote) = room_receiver.recv().await {
        let encoded = vote.to_bytes().unwrap();
        if let Err(e) = socket.send(Message::Binary(encoded)).await {
            eprintln!("Error sending vote: {}", e);
            break;
        }
    }
}
