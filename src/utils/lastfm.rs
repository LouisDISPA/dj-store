use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LastFmError {
    error: u16,
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

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    api_key: &'static str,
}
impl Client {
    pub fn new(api_key: &'static str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
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
        let result = match request.send().await {
            Ok(r) => r,
            Err(err) => {
                return LastFmResult::Err(LastFmError {
                    error: err.status().unwrap_or_default().as_u16(),
                    message: err.without_url().to_string(),
                });
            }
        };
        result.json().await.unwrap()
    }
}
