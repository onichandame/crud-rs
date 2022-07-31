use serde::Deserialize;

use crate::fixture::{get_db, get_schema, resolver::author::Author};

mod fixture;

#[tokio::test]
async fn simple_crud() {
    let db = get_db().await.unwrap();
    let schema = get_schema(db.clone()).await.unwrap();

    // create
    let response = schema
        .execute(
            r#"
        mutation {
            createAuthor(input:{name:"test"}){
                id name createdAt
            }
        }"#,
        )
        .await;
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CreateResponse {
        create_author: Author,
    }
    let author = serde_json::from_value::<CreateResponse>(response.data.into_json().unwrap())
        .unwrap()
        .create_author;
    assert_eq!(author.id, 1);
    assert_eq!(author.name, "test");
    assert!(author.created_at - chrono::Utc::now().naive_utc() < chrono::Duration::seconds(1));

    // update
    let response = schema
        .execute(
            r#"
    mutation{
        updateAuthors(filter:{id:{eq:1}},update:{name:"test2"})
    }"#,
        )
        .await;
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct UpdateResponse {
        update_authors: u64,
    }
    let updated = serde_json::from_value::<UpdateResponse>(response.data.into_json().unwrap())
        .unwrap()
        .update_authors;
    assert_eq!(updated, 1);

    // list
    let response = schema
        .execute(
            r#"
    query{
        authors(filter:{id:{eq:1}},paging:{first:1}){
            edges{ node{ id name createdAt } }
        }
    }
    "#,
        )
        .await;
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Edge {
        node: Author,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Authors {
        edges: Vec<Edge>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ListResponse {
        authors: Authors,
    }
    let authors = serde_json::from_value::<ListResponse>(response.data.into_json().unwrap())
        .unwrap()
        .authors
        .edges
        .into_iter()
        .map(|v| v.node)
        .collect::<Vec<_>>();
    assert_eq!(authors.len(), 1);
    let author = authors.get(0).unwrap();
    assert_eq!(author.name, "test2");
}
