use std::time::Duration;

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::Response,
    Error,
};
// use deku::{DekuContainerWrite, DekuUpdate, DekuWrite};
use serde::Serialize;
use tokio::{select, sync::broadcast::Receiver, time::timeout};
use uuid::Uuid;

use crate::utils::jwt::{self, Role};
use crate::utils::room_id::RoomID;

use super::state::ApiState;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct VoteEvent {
    pub music_id: Uuid,
    pub like: bool,
}

pub async fn handle_request(
    State(state): State<ApiState>,
    ws: WebSocketUpgrade,
    // Waiting to see if custom headers will be supported
    // on the WebSocket protocol
    // see: https://github.com/whatwg/websockets/issues/16
    //
    // user: User,
    Path(room_id): Path<RoomID>,
) -> Result<Response, (StatusCode, String)> {
    let mut rooms = state
        .rooms_channels
        .write()
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let room = rooms.entry(room_id).or_insert_with(|| {
        let (sender, _) = tokio::sync::broadcast::channel(100);
        sender
    });
    let receiver = room.subscribe();
    drop(rooms);

    Ok(ws.on_upgrade(move |socket| handle_room_websocket(socket, receiver)))
}

async fn handle_room_websocket(mut socket: WebSocket, mut room_receiver: Receiver<VoteEvent>) {
    // Check the first message is an admin auth token
    let future = timeout(Duration::from_secs(3), socket.recv());
    match future.await {
        Err(_) => {
            log::warn!("Socket auth timed out. (closing it)");
            let close_frame = CloseFrame {
                code: 4002,
                reason: "Auth timed out".into(),
            };
            socket.send(Message::Close(Some(close_frame))).await.ok();
            return;
        }
        Ok(msg) => {
            if !is_admin(msg) {
                let close_frame = CloseFrame {
                    code: 4001,
                    reason: "Auth Failed".into(),
                };
                socket.send(Message::Close(Some(close_frame))).await.ok();
                return;
            }
        }
    }

    loop {
        select! {
            msg = socket.recv() => {
                if matches!(msg, Some(Ok(Message::Close(_)) | Err(_)) | None) {
                    break;
                } else {
                    log::warn!("Received invalid websocket message: {:?}", msg);
                }
            }
            Ok(vote) = room_receiver.recv() => {
                let encoded = serde_json::to_string(&vote).unwrap();
                if let Err(e) = socket.send(Message::Text(encoded)).await {
                    log::error!("Error sending vote: {}", e);
                    break;
                }
            }
        }
    }
    log::info!("Admin disconnected from room");
}

#[allow(clippy::needless_return)]
fn is_admin(msg: Option<Result<Message, Error>>) -> bool {
    match msg {
        Some(Ok(Message::Text(token))) => {
            match jwt::verify(token.trim()) {
                Ok(user) if user.role == Role::Admin => {
                    log::info!("Admin connected to room");
                    return true;
                }
                Ok(user) => {
                    log::error!("Received non-admin auth: {:?}", user);
                    return false;
                }
                Err(e) => {
                    log::error!("Error verifying websocket auth token: {}", e);
                    return false;
                }
            };
        }
        Some(Err(e)) => {
            log::error!("Error receiving websocket auth token: {}", e);
            return false;
        }
        None => {
            log::warn!("Socket closed unexpectedly before receiving auth message");
            return false;
        }
        Some(Ok(msg)) => {
            log::error!("Received invalid auth message: {:?}", msg);
            return false;
        }
    }
}
