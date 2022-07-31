use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Author {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Title,
    Content,
    UserId,
}
