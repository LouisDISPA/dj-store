use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub use sea_orm_migration::prelude::MigratorTrait;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;

    #[tokio::test]
    async fn test_migrations() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to database");

        Migrator::up(&db, None)
            .await
            .expect("Failed to migrate database");

        Migrator::down(&db, None).await.expect("Failed to migrate database");
    }
}