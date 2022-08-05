use sea_orm_migration::prelude::*;

use super::tables::{Author, Post};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Post::UpdatedAt).date_time())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Content).string().not_null())
                    .col(ColumnDef::new(Post::AuthorId).integer().not_null())
                    .col(ColumnDef::new(Post::ParentId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::ParentId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Post::Table).to_owned())
            .await
    }
}
