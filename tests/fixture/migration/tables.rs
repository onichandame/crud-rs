use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Author {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Title,
    Content,
    AuthorId,
    ParentId,
}
