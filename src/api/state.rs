use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::Sender;

use crate::utils::room_id::RoomID;

use super::websocket::VoteEvent;

#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
    pub rooms_channels: Arc<RwLock<BTreeMap<RoomID, Sender<VoteEvent>>>>,
}

impl ApiState {
    pub fn init(db: DatabaseConnection) -> Self {
        Self {
            db,
            rooms_channels: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
