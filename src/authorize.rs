#[async_trait::async_trait]
pub trait Authorizer {
    /// returns a condition used to filter the records for all query actions
    async fn authorize(
        &self,
        _ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        Ok(sea_orm::Condition::all())
    }
}
