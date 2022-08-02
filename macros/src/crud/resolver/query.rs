use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{get_authorizer_constructor, get_filter_name, get_model, get_sort_name};

pub fn query_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let query_name: TokenStream = format!("{}Query", input.ident.to_string()).parse().unwrap();
    let list_fn_name: TokenStream =
        format!("{}", pluralizer::pluralize(&name.to_string(), 2, false))
            .to_lowercase()
            .parse()
            .unwrap();
    let filter_name = get_filter_name(input);
    let sort_name = get_sort_name(input);
    let authorizer_constructor = get_authorizer_constructor(input);
    let model = get_model(input);
    quote! {
        #[derive(Default)]
        pub struct #query_name;

        #[async_graphql::Object]
        impl #query_name {
            async fn #list_fn_name(
                &self,
                ctx: &async_graphql::Context<'_>,
                paging: Option<crud::Pagination>,
                filter: Option<#filter_name>,
                sorting: Option<Vec<#sort_name>>,
            ) -> async_graphql::Result<async_graphql::connection::Connection<crud::Cursor, #name>> {
                use crud::futures::prelude::*;
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let authorizer=#authorizer_constructor;
                let authorize_condition=crud::Authorizer::authorize(&authorizer,ctx).await?;
                let condition = filter.map_or(authorize_condition.clone(), |v| sea_orm::Condition::add(authorize_condition.clone(),v.build()));
                let query = <sea_orm::Select<#model::Entity> as sea_orm::QueryFilter>::filter(<#model::Entity as sea_orm::EntityTrait>::find(), condition);
                let count = <sea_orm::Select<#model::Entity> as sea_orm::PaginatorTrait<'_, sea_orm::DatabaseConnection>>::count(query.clone(), db).await?;
                let query = paging
                    .as_ref()
                    .map_or(Ok(query.clone()), |v| v.apply_pagination(query))?;
                let query = sorting.as_ref().map_or(query.clone(), |v| {
                    v.iter().fold(query, |query, v| v.apply_sort(query))
                });
                let mut connection = async_graphql::connection::Connection::new(
                    paging.as_ref().map_or(Ok(false), |v| v.has_prev())?,
                    paging
                        .as_ref()
                        .map_or(Ok(false), |v| v.has_next(count.try_into()?))?,
                );
                let start_index = paging.as_ref().map_or(Ok(0), |v| {
                    v.after
                        .as_ref()
                        .map_or(Ok(0), |v| <crud::Cursor as async_graphql::connection::CursorType>::decode_cursor(v).map(|v| v.offset))
                })? + 1;
                connection
                    .edges
                    .extend(
                        query
                            .clone()
                            .stream(db)
                            .await?
                            .enumerate()
                            .map(|(ind, val)| {
                                val.map(|v| {
                                    async_graphql::connection::Edge::new(
                                        crud::Cursor {
                                            offset: start_index + ind as u64, // may fail for extremely large ind. need optimization
                                        },
                                        v.into(),
                                    )
                                })
                                .map_err(|v| v.to_string())
                            })
                            .try_collect::<Vec<_>>()
                            .await?,
                    );
                Ok(connection)
            }
        }
    }
}
