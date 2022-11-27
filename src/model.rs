use chrono::prelude::*;
use uuid::Uuid;
use std::sync::RwLock;

use crate::api::room_id::RoomID;

pub struct Music {
    pub id: usize,
    pub title: String,
    pub artist: String,
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
}

#[derive(Debug, Clone, Copy)]
pub struct User {
    pub token: Uuid,
    pub room_id: RoomID,
}

pub static ROOMS: RwLock<Vec<Room>> = RwLock::new(Vec::new());
pub static USERS: RwLock<Vec<User>> = RwLock::new(Vec::new());

pub fn init() {
    let mut rooms = ROOMS.write().unwrap();

    let musics = vec![
        Music {
            id: 1,
            title: "Never Gonna Give You Up".to_string(),
            artist: "Rick Astley".to_string(),
        },
        Music {
            id: 2,
            title: "Sandstorm".to_string(),
            artist: "Darude".to_string(),
        },
        Music {
            id: 3,
            title: "Africa".to_string(),
            artist: "Toto".to_string(),
        },
    ];

    let room_id = "AAAAAA".parse().unwrap();
    let users = vec![
        User {
            token: Uuid::new_v4(),
            room_id,
        },
        User {
            token: Uuid::new_v4(),
            room_id,
        },
    ];
    let votes = vec![
        Vote {
            music_id: 1,
            user_id: users[0].token,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 2,
            user_id: users[1].token,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: users[1].token,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: users[0].token,
            datetime: Utc::now(),
        },
    ];

    rooms.push(Room {
        id: room_id,
        votes,
        musics,
    });
}
