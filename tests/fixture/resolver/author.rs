use async_graphql::SimpleObject;
use crud::{Hook, Relation, CRUD};
use serde::Deserialize;

use crate::fixture::entity;

#[derive(SimpleObject, CRUD, Relation, Hook, Deserialize, Debug)]
#[connection(
    name = "posts",
    target_dto = "super::post::Post",
    target_model = "entity::post",
    from = "id",
    to = "author_id"
)]
#[crud(model = "entity::author", subscribable, deletable)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct Author {
    pub id: i32,
    #[crud(creatable, updatable)]
    pub name: String,
}
