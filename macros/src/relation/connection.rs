use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helper::{get_meta_lists, get_value};

pub fn get_connections(input: &DeriveInput) -> Vec<TokenStream> {
    let connections = get_meta_lists(&input.attrs, "connection");
    connections
        .iter()
        .map(|connection| {
            let fn_name = get_value(connection, "name").unwrap();
            let target_dto=get_value(connection, "target_dto").unwrap();
            let filter_name:TokenStream=format!("{}Filter",target_dto.to_string()).parse().unwrap();
            let sort_name:TokenStream=format!("{}Sort",target_dto.to_string()).parse().unwrap();
            let target_model=get_value(connection, "target_model").unwrap();
            let from= get_value(connection, "from").unwrap();
            let to:TokenStream= get_value(connection, "to").unwrap().to_string().to_case(Case::Pascal).parse().unwrap();
            quote! {
                async fn #fn_name(
                    &self,
                    ctx: &async_graphql::Context<'_>,
                    paging: Option<crud::Pagination>,
                    filter: Option<#filter_name>,
                    sorting: Option<Vec<#sort_name>>,
                ) -> async_graphql::Result<async_graphql::connection::Connection<crud::Cursor, #target_dto>> {
                    use crud::futures::prelude::*;
                    use sea_orm::prelude::*;
                    let db = ctx.data::<DatabaseConnection>()?;
                    let query = #target_model ::Entity::find()
                        .filter(#target_model ::Column::#to .eq(self.#from))
                        .filter(<#target_dto as crud::Authorizer>::authorize(ctx).await?);
                    // filter
                    let query = match filter {
                        Some(filter) => query.filter(filter.build()),
                        None => query,
                    };
                    let count = query.clone().count(db).await?;
                    // pagination
                    let query = match paging.as_ref() {
                        Some(paging) => paging.apply_pagination(query)?,
                        None => query,
                    };
                    // sort
                    let query = match sorting.as_ref() {
                        Some(sortings) => sortings
                            .iter()
                            .fold(query, |query, sorting| sorting.apply_sort(query)),
                        None => query,
                    };
                    let (has_prev, has_next, offset) = match paging.as_ref() {
                        Some(paging) => (
                            paging.has_prev()?,
                            paging.has_next(count.try_into()?)?,
                            match paging.after.as_ref() {
                                Some(after) => {
                                    <crud::Cursor as async_graphql::connection::CursorType>::decode_cursor(
                                        after,
                                    )?
                                    .offset
                                }
                                None => 0,
                            },
                        ),
                        None => (false, false, 0),
                    };
                    let mut connection = async_graphql::connection::Connection::new(has_prev, has_next);
                    connection.edges.extend(
                        query
                            .stream(db)
                            .await?
                            .enumerate()
                            .map(|(ind, val)| {
                                val.map(|v| {
                                    async_graphql::connection::Edge::new(
                                        crud::Cursor {
                                            offset: offset + ind as u64 + 1,
                                        },
                                        v.into(),
                                    )
                                })
                            })
                            .map_err(|v| v.to_string())
                            .try_collect::<Vec<_>>()
                            .await?,
                    );
                    Ok(connection)
                }
            }
        })
        .collect()
}
