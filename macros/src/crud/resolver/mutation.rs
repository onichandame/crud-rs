use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::crud::helper::{get_filter_name, get_flag, get_metas, get_model, get_struct_fields};

pub fn mutation_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let mutation_name = format!("{}Mutation", &name).parse::<TokenStream>().unwrap();
    let model = get_model(input);
    let fields = get_struct_fields(input);
    let filter_name = get_filter_name(input);
    let (create_input_fields, create_input_transform_fields): (Vec<TokenStream>, Vec<TokenStream>) =
        fields
            .clone()
            .into_iter()
            .filter_map(|field| {
                if let Some(metas) = get_metas(&field.attrs) {
                    if get_flag(&metas, "creatable") {
                        let name = field.ident.expect("fields must be named");
                        let ty = field.ty;
                        Some((
                            quote! { pub #name: #ty },
                            quote! { #name: crud::IntoActiveValue::into_active_value(&self.#name) },
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unzip();
    let creatable = create_input_fields.len() > 0;
    let create_name: TokenStream = format!("create_{}", &name).to_lowercase().parse().unwrap();
    let create_input_name: TokenStream = format!("{}Input", &name).parse().unwrap();
    let create_input = if creatable {
        quote! {
            #[derive(async_graphql::InputObject)]
            pub struct #create_input_name {
                #(#create_input_fields),*
            }
            impl #create_input_name {
                fn into_active_model(&self) -> #model::ActiveModel {
                    #model::ActiveModel {
                        #(#create_input_transform_fields),* ,
                        ..Default::default()
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    let create_fn: TokenStream = if creatable {
        quote! {
            async fn #create_name(&self, ctx: &async_graphql::Context<'_>, input: #create_input_name) -> async_graphql::Result<#name> {
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let active_model = input.into_active_model();
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                let active_model=<#name as crud::Hook>::before_create(ctx,active_model,&txn).await?;
                let doc = sea_orm::ActiveModelTrait::insert(active_model,&txn).await?;
                txn.commit().await?;
                Ok(doc.into())
            }
        }
    } else {
        quote! {}
    };
    let (update_input_fields, update_input_transform_fields): (Vec<TokenStream>, Vec<TokenStream>) =
        fields
            .clone()
            .into_iter()
            .filter_map(|field| {
                if let Some(metas) = get_metas(&field.attrs) {
                    if get_flag(&metas, "updatable") {
                        let name = field.ident.expect("fields must be named");
                        let ty = field.ty;
                        let ty = match ty {
                            Type::Path(v) => {
                                let ident =
                                    v.path.segments.iter().fold(String::new(), |mut acc, v| {
                                        acc.push_str(&v.ident.to_string());
                                        acc.push(':');
                                        acc
                                    });
                                let optional_inner =
                                    vec!["Option:", "std:option:Option:", "core:option:Option:"]
                                        .into_iter()
                                        .find(|s| ident == *s)
                                        .and_then(|_| {
                                            v.path.segments.last().map(|v| v.arguments.clone())
                                        });
                                optional_inner.map_or(quote! {Option<#v>}, |v| {
                                    quote! {async_graphql::MaybeUndefined #v}
                                })
                            }
                            _other => {
                                panic!("field type must be path")
                            }
                        };
                        Some((
                            quote! { pub #name: #ty },
                            quote! { #name: crud::IntoActiveValue::into_active_value(&self.#name) },
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unzip();
    let updatable = update_input_fields.len() > 0;
    let update_name: TokenStream = format!(
        "update_{}",
        pluralizer::pluralize(&name.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let update_input_name: TokenStream = format!("{}Update", &name.to_string()).parse().unwrap();
    let update_input = if updatable {
        quote! {
            #[derive(async_graphql::InputObject)]
            pub struct #update_input_name {
                #(#update_input_fields),*
            }
            impl #update_input_name {
                fn into_active_model(&self) -> #model::ActiveModel {
                    #model::ActiveModel {
                        #(#update_input_transform_fields),* ,
                        ..Default::default()
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    let update_fn: TokenStream = if updatable {
        quote! {
            async fn #update_name(&self, ctx: &async_graphql::Context<'_>,filter:Option<#filter_name>, update: #update_input_name) -> async_graphql::Result<u64> {
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let active_model = update.into_active_model();
                let authorize_condition=<#name as crud::Authorizer>::authorize(ctx).await?;
                let condition = filter.map_or(authorize_condition.clone(),|v| sea_orm::Condition::add(authorize_condition.clone(),v.build()));
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                let active_model=<#name as crud::Hook>::before_update(ctx,condition.clone(),active_model,&txn).await?;
                let result=sea_orm::UpdateMany::exec(
                    <sea_orm::UpdateMany<#model::Entity> as sea_orm::QueryFilter>::filter(
                        sea_orm::UpdateMany::set(
                            sea_orm::EntityTrait::update_many(),
                            active_model,
                        ),
                        condition.clone(),
                    ),
                    &txn,
                )
                .await?;
                txn.commit().await?;
                Ok(result.rows_affected)
            }
        }
    } else {
        quote! {}
    };
    let deletable = get_flag(&get_metas(&input.attrs).unwrap(), "deletable");
    let delete_name: TokenStream = format!(
        "delete_{}",
        pluralizer::pluralize(&name.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let delete_fn = if deletable {
        quote! {
        async fn #delete_name(
                &self,
                ctx: &async_graphql::Context<'_>,
                filter: #filter_name,
            ) -> async_graphql::Result<u64> {
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let authorize_condition=<#name as crud::Authorizer>::authorize(ctx).await?;
                let condition = sea_orm::Condition::add(authorize_condition,filter.build());
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                <#name as crud::Hook>::before_delete(ctx,condition.clone(),&txn).await?;
                let result=sea_orm::DeleteMany::exec(
                    <sea_orm::DeleteMany<#model::Entity> as sea_orm::QueryFilter>::filter(
                        sea_orm::EntityTrait::delete_many(),
                        condition.clone(),
                    ),
                    &txn,
                )
                .await?;
                txn.commit().await?;
                Ok(result.rows_affected)
            }
        }
    } else {
        quote! {}
    };
    let mutable = creatable || updatable || deletable;
    if mutable {
        quote! {
            #[derive(Default)]
            pub struct #mutation_name;

            #create_input
            #update_input

            #[async_graphql::Object]
            impl #mutation_name {
                #create_fn
                #update_fn
                #delete_fn
            }

        }
    } else {
        quote! {}
    }
}
