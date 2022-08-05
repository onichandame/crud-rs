use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

use crate::fixture::{
    entity, get_db, get_schema, request,
    resolver::{author::Author, post::Post},
};

mod fixture;

#[tokio::test]
pub async fn many_to_one() {
    let db = get_db().await.unwrap();
    let schema = get_schema(db.clone()).await.unwrap();
    // prepare data
    let author = entity::author::ActiveModel {
        name: Set("test".to_owned()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let post1 = entity::post::ActiveModel {
        author_id: Set(author.id),
        title: Set("post".to_owned()),
        content: Set("post".to_owned()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let post2 = entity::post::ActiveModel {
        author_id: Set(author.id),
        title: Set("post2".to_owned()),
        content: Set("post2".to_owned()),
        parent_id: Set(Some(post1.id)),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    // query many-to-one
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Node {
        id: i32,
        author: Author,
        parent: Option<Post>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Edge {
        node: Node,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Posts {
        edges: Vec<Edge>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ListResponse {
        posts: Posts,
    }
    let posts = request::<ListResponse>(
        &schema,
        r#"
    query {
        posts{
            edges{
                node{
                    id
                    author{
                        id name
                    }
                    parent{
                        id createdAt updatedAt title content parentId authorId
                    }
                }
            }
        }
    }
    "#,
    )
    .await
    .posts
    .edges
    .into_iter()
    .map(|v| v.node)
    .collect::<Vec<_>>();
    assert!(posts.len() > 0);
    let post1_response = posts.iter().find(|v| v.id == post1.id).unwrap();
    let post2_response = posts.iter().find(|v| v.id == post2.id).unwrap();
    assert_eq!(&post1_response.author.name, &author.name);
    assert_eq!(&post2_response.author.name, &author.name);
    assert_eq!(
        &post2_response.parent.as_ref().unwrap().id,
        &post1_response.id
    );
}
