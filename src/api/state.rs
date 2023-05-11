use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use argon2::PasswordHash;
use deezer_rs::Deezer;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::utils::room_id::RoomID;

use super::websocket::VoteEvent;

#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
    pub deezer_client: Deezer,
    pub rooms_channels: RoomChannels,
    // TODO: Use global static variable instead of Arc again is better ?
    // the admin_info is only inizialized once, and cannot be changed
    pub admin_info: Arc<AdminInfo>,
}

impl ApiState {
    pub fn new(db: DatabaseConnection, admin_username: String, password_hash: String) -> Self {
        let client = Deezer::new();
        let admin_info = AdminInfo::new(admin_username, password_hash);

        Self {
            db,
            deezer_client: client,
            rooms_channels: RoomChannels::new(),
            admin_info: Arc::new(admin_info),
        }
    }
}

pub struct AdminInfo {
    pub username: String,
    pub password: PasswordHash<'static>,
}

impl AdminInfo {
    fn new(username: String, password: String) -> Self {
        let password = Box::leak(password.into_boxed_str());
        let password = PasswordHash::new(password).expect("Failed to hash password");
        AdminInfo { username, password }
    }
}

#[derive(Clone)]
pub struct RoomChannels {
    channels: Arc<RwLock<BTreeMap<RoomID, Sender<VoteEvent>>>>,
}

impl RoomChannels {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn send_vote(&self, room_id: RoomID, vote: VoteEvent) {
        let channels = match self.channels.read() {
            Ok(channels) => channels,
            Err(_) => {
                log::error!("Failed to send vote to room {}: Poisoned lock", room_id);
                return;
            }
        };
        if let Some(channel) = channels.get(&room_id) {
            if let Err(err) = channel.send(vote) {
                log::warn!("Failed to send vote to room {}: {}", room_id, err);
            }
        }
    }

    pub fn subscribe(&self, room_id: RoomID) -> Option<ReceiverGuard> {
        let mut channels = self
            .channels
            .write()
            .map_err(|err| {
                log::error!("Failed to subscribe to room {}: {}", room_id, err);
            })
            .ok()?;

        let channel = channels.entry(room_id).or_insert_with(|| {
            let (sender, _) = tokio::sync::broadcast::channel(100);
            sender
        });

        let receiver = channel.subscribe();
        Some(ReceiverGuard {
            room_id,
            receiver: Some(receiver),
            guard: self.clone(),
        })
    }
}

pub struct ReceiverGuard {
    room_id: RoomID,
    receiver: Option<Receiver<VoteEvent>>,
    guard: RoomChannels,
}

impl Deref for ReceiverGuard {
    type Target = Receiver<VoteEvent>;

    fn deref(&self) -> &Self::Target {
        self.receiver.as_ref().unwrap()
    }
}

impl DerefMut for ReceiverGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.receiver.as_mut().unwrap()
    }
}

impl Drop for ReceiverGuard {
    fn drop(&mut self) {
        match self.guard.channels.write() {
            Ok(mut channels) => {
                if let Some(channel) = channels.get_mut(&self.room_id) {
                    if channel.receiver_count() == 1 {
                        log::debug!("Closing channel for room {}", self.room_id);
                        channels.remove(&self.room_id);
                    }
                }
                // make sure to drop the receiver before the channels lock
                drop(self.receiver.take());
                drop(channels);
            }
            Err(_) => {
                log::error!(
                    "Failed to close channel for room {}: Poisoned lock",
                    self.room_id
                );
            }
        }
    }
}
