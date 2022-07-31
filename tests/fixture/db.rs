use std::error::Error;

use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use super::migration::Migrator;

pub async fn get_db() -> Result<DatabaseConnection, Box<dyn Error + Sync + Send>> {
    let db = sea_orm::Database::connect("sqlite::memory:").await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
