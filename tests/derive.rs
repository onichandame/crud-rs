use std::error::Error;

use async_graphql::{MergedObject, MergedSubscription, SimpleObject};
use crud::CRUD;
use migration::Migrator;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

mod entity;
mod migration;

#[tokio::test]
async fn simple_crud() {
    let db = get_db().await.unwrap();
    let schema = get_schema(db.clone()).await.unwrap();
    let response = schema
        .execute(r#"mutation {createAuthor(input:{name:"test"}){id name}}"#)
        .await;
    println!("{:?}", &response);
}

#[derive(SimpleObject, CRUD)]
#[crud(model = "entity::author", subscribable)]
struct Author {
    id: i32,
    #[crud(creatable, updatable)]
    name: String,
}

#[derive(Default, MergedObject)]
struct Query(AuthorQuery);
#[derive(Default, MergedObject)]
struct Mutation(AuthorMutation);
#[derive(Default, MergedSubscription)]
struct Subscription(AuthorSubscription);

type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

async fn get_db() -> Result<DatabaseConnection, Box<dyn Error + Sync + Send>> {
    let db = sea_orm::Database::connect("sqlite::memory:").await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

async fn get_schema(db: DatabaseConnection) -> Result<Schema, Box<dyn Error + Send + Sync>> {
    let schema = async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(db)
    .finish();
    Ok(schema)
}
