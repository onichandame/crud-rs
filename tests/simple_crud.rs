use serde::Deserialize;

use crate::fixture::{get_db, get_schema, request, resolver::author::Author};

mod fixture;

#[tokio::test]
async fn simple_crud() {
    let db = get_db().await.unwrap();
    let schema = get_schema(db.clone()).await.unwrap();

    // create
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CreateResponse {
        create_author: Author,
    }
    let author = request::<CreateResponse>(
        &schema,
        r#"
        mutation {
            createAuthor(input:{name:"test"}){
                id name
            }
        }"#,
    )
    .await
    .create_author;
    assert_eq!(author.id, 1);
    assert_eq!(author.name, "test");

    // update&
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct UpdateResponse {
        update_authors: u64,
    }
    let updated = request::<UpdateResponse>(
        &schema,
        r#"
    mutation{
        updateAuthors(filter:{id:{eq:1}},update:{name:"test2"})
    }"#,
    )
    .await
    .update_authors;
    assert_eq!(updated, 1);

    // list
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
    let authors = request::<ListResponse>(
        &schema,
        r#"
    query{
        authors(filter:{id:{eq:1}},paging:{first:1}){
            edges{ node{ id name } }
        }
    }
    "#,
    )
    .await
    .authors
    .edges
    .into_iter()
    .map(|v| v.node)
    .collect::<Vec<_>>();
    assert_eq!(authors.len(), 1);
    let author = authors.get(0).unwrap();
    assert_eq!(author.name, "test2");
    // delete
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DeleteResponse {
        delete_authors: u64,
    }
    let delete = || async {
        request::<DeleteResponse>(
            &schema,
            r#"
    mutation{
        deleteAuthors(filter:{id:{eq:1}})
    }
    "#,
        )
        .await
        .delete_authors
    };
    assert_eq!(delete().await, 1);
    assert_eq!(delete().await, 0);
}
