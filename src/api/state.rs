use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
}

impl ApiState {
    pub fn init(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
