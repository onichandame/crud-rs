use async_graphql::{async_trait::async_trait, SimpleObject};
use chrono::NaiveDateTime;
use crud::{Authorizer, Hook, Relation, CRUD};
use sea_orm::Set;
use serde::Deserialize;

use crate::fixture::entity;

#[derive(SimpleObject, CRUD, Relation, Deserialize, Debug)]
#[relation(
    name = "author",
    target_dto = "super::author::Author",
    target_model = "entity::author",
    from = "author_id",
    to = "id"
)]
#[relation(
    name = "parent",
    target_dto = "Post",
    target_model = "entity::post",
    from = "parent_id",
    to = "id",
    nullable
)]
#[connection(
    name = "children",
    target_dto = "Post",
    target_model = "entity::post",
    from = "id",
    to = "parent_id"
)]
#[crud(model = "entity::post", subscribable, deletable)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct Post {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub content: String,
    #[crud(creatable)]
    pub author_id: i32,
    #[crud(creatable)]
    pub parent_id: Option<i32>,
}

#[async_trait]
impl Hook for Post {
    type ActiveModel = entity::post::ActiveModel;
    async fn before_create(
        _ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.created_at = Set(chrono::Utc::now().naive_utc());
        Ok(input)
    }
    async fn before_update(
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(input)
    }
}

impl Authorizer for Post {}
