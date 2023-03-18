use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use deezer_rs::{search::SearchService, track::TrackService, Deezer};
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::Sender;

use crate::utils::room_id::RoomID;

use super::websocket::VoteEvent;

#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
    pub search_client: Arc<SearchService>,
    pub tracks_client: Arc<TrackService>,
    pub rooms_channels: Arc<RwLock<BTreeMap<RoomID, Sender<VoteEvent>>>>,
}

impl ApiState {
    pub fn init(db: DatabaseConnection) -> Self {
        let client = Deezer::new();
        Self {
            db,
            search_client: Arc::new(client.search),
            tracks_client: Arc::new(client.track),
            rooms_channels: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
