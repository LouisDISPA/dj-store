use sea_orm::DatabaseConnection;

use crate::utils::lastfm::Client;


#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
    pub client: Client,
}

impl ApiState {
    pub fn init(db: DatabaseConnection, api_key: String) -> Self {
        let api_key = Box::leak(api_key.into_boxed_str());
        let client = Client::new(api_key);

        Self { db, client }
    }
}