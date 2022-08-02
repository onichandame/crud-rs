use std::error::Error;

use sea_orm::DatabaseConnection;

use super::resolver::{Mutation, Query, Subscription};

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

pub async fn get_schema(db: DatabaseConnection) -> Result<Schema, Box<dyn Error + Send + Sync>> {
    let schema = async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(db)
    .finish();
    Ok(schema)
}
