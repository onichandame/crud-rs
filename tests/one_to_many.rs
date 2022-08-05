use std::ops::Deref;

use crate::fixture::{get_db, get_schema, request};

use fixture::entity;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

mod fixture;

#[tokio::test]
async fn one_to_many() {
    let db = get_db().await.unwrap();
    let schema = get_schema(db.clone()).await.unwrap();
    // prepare data
    let author = entity::author::ActiveModel {
        name: Set("author".to_owned()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let post1 = entity::post::ActiveModel {
        title: Set("post1".to_owned()),
        content: Set("post1".to_owned()),
        author_id: Set(author.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let post11 = entity::post::ActiveModel {
        title: Set("post11".to_owned()),
        content: Set("post11".to_owned()),
        author_id: Set(author.id),
        parent_id: Set(Some(post1.id)),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    let post12 = entity::post::ActiveModel {
        title: Set("post12".to_owned()),
        content: Set("post12".to_owned()),
        author_id: Set(author.id),
        parent_id: Set(Some(post1.id)),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    // query one-to-many
    // TODO post.children
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PostChildrenResponse {
        edges: Vec<PostEdge>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PostNode {
        id: i32,
        title: String,
        children: Option<PostChildrenResponse>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PostEdge {
        node: PostNode,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PostResponse {
        edges: Vec<PostEdge>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AuthorNode {
        posts: PostResponse,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AuthorEdge {
        node: AuthorNode,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Authors {
        edges: Vec<AuthorEdge>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ListResponse {
        authors: Authors,
    }
    let authors = request::<ListResponse>(
        &schema,
        r#"
    query {
        authors{
            edges{
                node{
                    posts(filter:{parentId:{null:true}}){
                        edges{
                            node{
                                id createdAt updatedAt title content parentId authorId
                                children{
                                    edges{
                                        node{
                                            id createdAt updatedAt title content parentId authorId
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
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
    let posts = authors
        .get(0)
        .unwrap()
        .posts
        .edges
        .iter()
        .map(|v| &v.node)
        .collect::<Vec<_>>();
    assert_eq!(posts.len(), 1);
    let root_post = posts.get(0).unwrap();
    assert_eq!(root_post.id, post1.id);
    let children = root_post
        .children
        .as_ref()
        .unwrap()
        .edges
        .iter()
        .map(|v| &v.node)
        .collect::<Vec<_>>();
    assert_eq!(children.len(), 2);
    let post11_response = children.iter().find(|v| v.id == post11.id).unwrap().deref();
    let post12_response = children.iter().find(|v| v.id == post12.id).unwrap().deref();
    assert_eq!(post11_response.title, post11.title);
    assert_eq!(post12_response.title, post12.title);
}
