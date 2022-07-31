use async_graphql::{MergedObject, MergedSubscription};

pub mod author;

use self::author::{AuthorMutation, AuthorQuery, AuthorSubscription};

#[derive(Default, MergedObject)]
pub struct Query(AuthorQuery);
#[derive(Default, MergedObject)]
pub struct Mutation(AuthorMutation);
#[derive(Default, MergedSubscription)]
pub struct Subscription(AuthorSubscription);
