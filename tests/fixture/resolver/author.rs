use async_graphql::{async_trait::async_trait, SimpleObject};
use chrono::NaiveDateTime;
use crud::{Hook, CRUD};
use sea_orm::Set;
use serde::Deserialize;

use crate::fixture::entity;

#[derive(SimpleObject, CRUD, Deserialize, Debug)]
#[crud(
    model = "entity::author",
    hook = "AuthorHook::default()",
    subscribable,
    deletable
)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: i32,
    #[crud(creatable, updatable)]
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default)]
struct AuthorHook {}

#[async_trait]
impl Hook for AuthorHook {
    type ActiveModel = entity::author::ActiveModel;
    async fn before_create(
        &self,
        _ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.created_at = Set(chrono::Utc::now().naive_utc());
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(input)
    }
}
