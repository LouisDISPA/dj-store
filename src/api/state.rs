use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use argon2::PasswordHash;
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
    pub admin_info: Arc<AdminInfo>,
}

impl ApiState {
    pub fn init(db: DatabaseConnection, admin_username: String, password_hash: String) -> Self {
        let client = Deezer::new();
        let admin_info = AdminInfo::new(admin_username, password_hash);

        Self {
            db,
            search_client: Arc::new(client.search),
            tracks_client: Arc::new(client.track),
            rooms_channels: Arc::new(RwLock::new(BTreeMap::new())),
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

        // This is safe because ADMIN_INFO is only initialized once at the start of the program
        AdminInfo { username, password }
    }
}
