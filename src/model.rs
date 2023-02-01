use chrono::prelude::*;
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};
use tokio::sync::broadcast::{self, Sender};
use uuid::Uuid;

use crate::{api::room_id::RoomID, utils::jwt};

pub struct Music {
    pub title: String,
    pub artist: String,
    pub votes: usize,
}

pub struct Vote {
    pub music_id: usize,
    pub user_id: Uuid,
    pub datetime: DateTime<Utc>,
}

pub struct Room {
    pub id: RoomID,
    pub votes: Vec<Vote>,
    pub musics: Vec<Music>,
    pub musics_to_id: HashMap<String, usize>,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub active: bool,
    pub channel: Sender<VoteEvent>,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
pub struct VoteEvent {
    #[deku(bits = "32")]
    pub music_id: usize,
    #[deku(bits = "32")]
    pub votes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum Role {
    Admin,
    User { room_id: RoomID },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct User {
    pub uid: Uuid,
    #[serde(flatten)]
    pub role: Role,
}

pub static ROOMS: RwLock<Vec<Room>> = RwLock::new(Vec::new());
pub static USERS: RwLock<Vec<User>> = RwLock::new(Vec::new());

pub fn init() {
    let mut rooms = ROOMS.write().unwrap();
    let mut users = USERS.write().unwrap();

    let musics = vec![
        Music {
            title: "Never Gonna Give You Up".to_string(),
            artist: "Rick Astley".to_string(),
            votes: 0,
        },
        Music {
            title: "Sandstorm".to_string(),
            artist: "Darude".to_string(),
            votes: 3,
        },
        Music {
            title: "Africa".to_string(),
            artist: "Toto".to_string(),
            votes: 1,
        },
    ];

    let musics_to_id = musics
        .iter()
        .enumerate()
        .map(|(i, m)| (format!("{} - {}", m.artist, m.title), i))
        .collect();

    let room_id = "AAAAAA".parse().unwrap();

    let admin = User {
        uid: Uuid::new_v4(),
        role: Role::Admin,
    };
    println!("Admin token: {}", jwt::sign(admin));

    *users = vec![
        admin,
        User {
            uid: Uuid::new_v4(),
            role: Role::User { room_id },
        },
        User {
            uid: Uuid::new_v4(),
            role: Role::User { room_id },
        },
    ];
    let votes = vec![
        Vote {
            music_id: 1,
            user_id: users[0].uid,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 2,
            user_id: users[1].uid,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: users[1].uid,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: users[0].uid,
            datetime: Utc::now(),
        },
    ];

    rooms.push(Room {
        id: room_id,
        votes,
        musics,
        musics_to_id,
        creation: Utc::now(),
        expiration: Utc::now() + chrono::Duration::days(1),
        active: true,
        channel: broadcast::channel(10).0,
    });
}
