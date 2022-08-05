use async_graphql::async_trait::async_trait;
use sea_orm::ActiveModelTrait;

#[async_trait]
pub trait Hook {
    type ActiveModel: ActiveModelTrait + Send;
    async fn before_create(
        _ctx: &async_graphql::Context<'_>,
        input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        Ok(input)
    }
    async fn before_update(
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        Ok(input)
    }
    async fn before_delete(
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<()> {
        Ok(())
    }
}
