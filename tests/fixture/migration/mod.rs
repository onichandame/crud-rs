pub use sea_orm_migration::prelude::*;

mod m20220722_000001_create_table_author;
mod m20220722_000002_create_table_post;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220722_000001_create_table_author::Migration),
            Box::new(m20220722_000002_create_table_post::Migration),
        ]
    }
}
