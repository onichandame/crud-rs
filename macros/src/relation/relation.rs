use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helper::{get_flag, get_meta_lists, get_str_literal, get_value};

pub fn get_relations(input: &DeriveInput) -> Vec<TokenStream> {
    let relations = get_meta_lists(&input.attrs, "relation");
    relations
        .iter()
        .map(|relation| {
            let fn_name = get_value(relation, "name").unwrap();
            let fn_name_lit = get_str_literal(&fn_name);
            let from = get_value(relation, "from").unwrap();
            let to: TokenStream = get_value(relation, "to")
                .unwrap()
                .to_string()
                .to_case(Case::Pascal)
                .parse()
                .unwrap();
            let target_dto = get_value(relation, "target_dto").unwrap();
            let target_model = get_value(relation, "target_model").unwrap();
            let nullable = get_flag(relation, "nullable");
            let (return_type, null_check) = if nullable {
                (quote! {Option<#target_dto>}, quote! {})
            } else {
                (
                    quote! {#target_dto},
                    quote! {
                            .ok_or(format!("{} not found",#fn_name_lit))?
                    },
                )
            };
            quote! {
            async fn #fn_name(
                &self,
                ctx: &async_graphql::Context<'_>,
            ) -> async_graphql::Result<#return_type> {
                use sea_orm::prelude::*;
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let authorize_condition = <#target_dto as crud::Authorizer>::authorize(ctx).await?;
                let data=#target_model ::Entity::find()
                    .filter(#target_model::Column::#to .eq(self. #from))
                    .filter(authorize_condition)
                    .one(db)
                    .await?
                    .map(|v|#target_dto::from(v));
                Ok(data
                    #null_check
                    .into())
            }
            }
        })
        .collect()
}
