use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LastFmError {
    error: i32,
    message: String,
}

impl Display for LastFmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, message: {}", self.error, self.message)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LastFmResult {
    Ok { results: LastFmSearchResult },
    Err(LastFmError),
}

impl LastFmSearchResult {
    pub fn into_tracs(self) -> Vec<LastFmTrack> {
        self.trackmatches.track
    }
}

#[derive(Serialize, Deserialize)]
pub struct LastFmSearchResult {
    trackmatches: LastFmTrackMatches,
}

#[derive(Serialize, Deserialize)]
struct LastFmTrackMatches {
    track: Vec<LastFmTrack>,
}

#[derive(Serialize, Deserialize)]
pub struct LastFmTrack {
    pub name: String,
    pub artist: String,
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    api_key: String,
}
impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn search(&self, query: &str) -> LastFmResult {
        let request = self
            .client
            .get("http://ws.audioscrobbler.com/2.0/")
            .query(&[
                ("method", "track.search"),
                ("api_key", &self.api_key),
                ("format", "json"),
                ("track", query),
            ]);
        let result = request.send().await.unwrap();
        result.json().await.unwrap()
    }
}
