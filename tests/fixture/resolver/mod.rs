use async_graphql::{MergedObject, MergedSubscription};

pub mod author;
pub mod post;

use self::{
    author::{AuthorMutation, AuthorQuery, AuthorSubscription},
    post::{PostMutation, PostQuery, PostSubscription},
};

#[derive(Default, MergedObject)]
pub struct Query(AuthorQuery, PostQuery);
#[derive(Default, MergedObject)]
pub struct Mutation(AuthorMutation, PostMutation);
#[derive(Default, MergedSubscription)]
pub struct Subscription(AuthorSubscription, PostSubscription);
