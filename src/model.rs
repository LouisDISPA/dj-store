use chrono::prelude::*;
use std::{str::FromStr, sync::RwLock};

pub struct Music {
    pub id: usize,
    pub title: String,
    pub artist: String,
}

pub struct Vote {
    pub music_id: usize,
    pub user_id: usize,
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomID {
    pub id: u32,
}

pub struct Room {
    pub id: RoomID,
    pub votes: Vec<Vote>,
    pub users: Vec<User>,
    pub musics: Vec<Music>,
}

pub struct User {
    pub id: usize,
    pub token: String,
}

pub static ROOMS: RwLock<Vec<Room>> = RwLock::new(Vec::new());

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
            id: 1,
            token: "token1".to_string(),
        },
        User {
            id: 2,
            token: "token2".to_string(),
        },
    ];
    let votes = vec![
        Vote {
            music_id: 1,
            user_id: 1,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 2,
            user_id: 2,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: 2,
            datetime: Utc::now(),
        },
        Vote {
            music_id: 1,
            user_id: 3,
            datetime: Utc::now(),
        },
    ];

    rooms.push(Room {
        id: room_id,
        votes,
        users,
        musics,
    });
}

impl FromStr for RoomID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(());
        }

        let mut id = 0;
        for (i, c) in s.chars().enumerate() {
            // check if the character is between A and Z
            if ('A'..='Z').contains(&c) {
                // convert the character to a number
                let num = c as u32 - 'A' as u32;
                // add the number to the id
                id += num * 26u32.pow(i as u32);
            } else {
                return Err(());
            }
        }
        Ok(RoomID { id })
    }
}

impl ToString for RoomID {
    fn to_string(&self) -> String {
        let mut id = self.id;
        let mut s = String::new();
        while id > 0 {
            s.push((('A' as u32) + (id % 26)) as u8 as char);
            id /= 26;
        }
        s
    }
}
