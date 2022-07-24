use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::crud::helper::{
    get_meta_field, get_meta_list, get_metas, get_object_name, get_struct_fields, has_meta,
};

pub fn resolver_expand(input: &DeriveInput) -> TokenStream {
    let query_name = get_object_name(input.attrs.clone(), "resolver").map_or(
        format!("{}Query", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let mutation_name = get_object_name(input.attrs.clone(), "resolver").map_or(
        format!("{}Mutation", input.ident.to_string())
            .parse()
            .unwrap(),
        |v| v,
    );
    let subscription_name = get_object_name(input.attrs.clone(), "resolver").map_or(
        format!("{}Subscription", input.ident.to_string())
            .parse()
            .unwrap(),
        |v| v,
    );
    let list_name: TokenStream = format!(
        "list_{}",
        pluralizer::pluralize(&input.ident.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let create_name: TokenStream = format!("create_{}", &input.ident.to_string())
        .to_lowercase()
        .parse()
        .unwrap();
    let update_name: TokenStream = format!(
        "update_{}",
        pluralizer::pluralize(&input.ident.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let delete_name: TokenStream = format!(
        "delete_{}",
        pluralizer::pluralize(&input.ident.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let stream_name: TokenStream = format!(
        "stream_{}",
        pluralizer::pluralize(&input.ident.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let name = input.ident.clone();
    let filter_name = get_object_name(input.attrs.clone(), "filter").map_or(
        format!("{}Filter", input.ident.to_string())
            .parse()
            .unwrap(),
        |v| v,
    );
    let sort_name = get_object_name(input.attrs.clone(), "sort").map_or(
        format!("{}Sort", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let default_authorizer_name: TokenStream =
        format!("_DefaultAuthorizer{}", query_name.to_string())
            .parse()
            .unwrap();
    let default_authorizer_constructor: TokenStream =
        format!("{}::new()", default_authorizer_name.to_string())
            .parse()
            .unwrap();
    let authorizer_constructor = get_object_name(input.attrs.clone(), "authorizer")
        .map_or(default_authorizer_constructor.clone(), |v| v);
    let default_hook_name: TokenStream = format!("_DefaultHook{}", query_name.to_string())
        .parse()
        .unwrap();
    let default_hook_constructor: TokenStream = format!("{}::new()", default_hook_name.to_string())
        .parse()
        .unwrap();
    let hook_constructor = get_object_name(input.attrs.clone(), "hook")
        .map_or(default_hook_constructor.clone(), |v| v);
    let model = get_meta_field(&get_metas(&input.attrs).unwrap(), "model").unwrap();
    let deletable = has_meta(&get_metas(&input.attrs).unwrap(), "deletable");
    let fields = get_struct_fields(&input.data).unwrap();
    let impl_from: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .map(|v| {
            let name = v.ident.expect("fields must be named");
            quote! {#name: _self.#name}
        })
        .collect();
    let create_input_fields: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            let name = field.ident.expect("fields must be named");
            let ty = field.ty;
            if let Ok(meta) = get_meta_list(field.attrs, "crud") {
                if has_meta(&meta, "creatable") {
                    Some(quote! { pub #name: #ty })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let creatable = create_input_fields.len() > 0;
    let create_input_transform_fields: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            let name = field.ident.expect("fields must be named");
            if let Ok(meta) = get_meta_list(field.attrs, "crud") {
                if has_meta(&meta, "creatable") {
                    Some(quote! { #name: crud::IntoActiveValue::into_active_value(&self.#name) })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let create_input_name: TokenStream = format!("{}Input", &input.ident.to_string())
        .parse()
        .unwrap();
    let create_input = if creatable {
        quote! {
            #[derive(async_graphql::InputObject)]
            pub struct #create_input_name {
                #(#create_input_fields),*
            }
        }
    } else {
        quote! {}
    };
    let create_input_transform = if creatable {
        quote! {
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
                let hooks=#hook_constructor;
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                let active_model=crud::Hook::before_create(&hooks,ctx,active_model,&txn).await?;
                let doc = sea_orm::ActiveModelTrait::insert(active_model,&txn).await?;
                txn.commit().await?;
                Ok(doc.into())
            }
        }
    } else {
        quote! {}
    };
    let update_input_fields: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            let name = field.ident.expect("fields must be named");
            let ty = field.ty;
            let ty = match ty {
                Type::Path(v) => {
                    let ident = v.path.segments.iter().fold(String::new(), |mut acc, v| {
                        acc.push_str(&v.ident.to_string());
                        acc.push(':');
                        acc
                    });
                    let optional_inner =
                        vec!["Option:", "std:option:Option:", "core:option:Option:"]
                            .into_iter()
                            .find(|s| ident == *s)
                            .and_then(|_| v.path.segments.last().map(|v| v.arguments.clone()));
                    optional_inner.map_or(quote! {Option<#v>}, |v| {
                        quote! {async_graphql::MaybeUndefined #v}
                    })
                }
                _other => {
                    panic!("field type must be path")
                }
            };
            if let Ok(meta) = get_meta_list(field.attrs, "crud") {
                if has_meta(&meta, "updatable") {
                    Some(quote! { pub #name: #ty })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let updatable = update_input_fields.len() > 0;
    let update_input_transform_fields: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            let name = field.ident.expect("fields must be named");
            if let Ok(meta) = get_meta_list(field.attrs, "crud") {
                if has_meta(&meta, "updatable") {
                    Some(quote! { #name: crud::IntoActiveValue::into_active_value(&self.#name) })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let update_input_name: TokenStream = format!("{}Update", &input.ident.to_string())
        .parse()
        .unwrap();
    let update_input = if updatable {
        quote! {
            #[derive(async_graphql::InputObject)]
            pub struct #update_input_name {
                #(#update_input_fields),*
            }
        }
    } else {
        quote! {}
    };
    let update_input_transform = if updatable {
        quote! {
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
                let hooks=#hook_constructor;
                let authorizer=#authorizer_constructor;
                let authorize_condition=crud::Authorizer::authorize(&authorizer,ctx).await?;
                let condition = filter.map_or(authorize_condition.clone(),|v| sea_orm::Condition::add(authorize_condition.clone(),v.build()));
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                let active_model=crud::Hook::before_update(&hooks,ctx,condition.clone(),active_model,&txn).await?;
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
    let delete_fn = if deletable {
        quote! {
        async fn #delete_name(
                &self,
                ctx: &async_graphql::Context<'_>,
                filter: #filter_name,
            ) -> async_graphql::Result<u64> {
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let authorizer=#authorizer_constructor;
                let authorize_condition=crud::Authorizer::authorize(&authorizer,ctx).await?;
                let condition = sea_orm::Condition::add(authorize_condition,filter.build());
                let hooks=#hook_constructor;
                let txn=sea_orm::TransactionTrait::begin(db).await?;
                crud::Hook::before_delete(&hooks,ctx,condition.clone(),&txn).await?;
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
    let mutation_struct = if mutable {
        quote! {
        #[derive(Default)]
        pub struct #mutation_name;
        }
    } else {
        quote! {}
    };
    let mutation_impl = if mutable {
        quote! {
            #[async_graphql::Object]
            impl #mutation_name {
                #create_fn
                #update_fn
                #delete_fn
            }

        }
    } else {
        quote! {}
    };
    quote! {
        #[derive(Default)]
        pub struct #query_name;
        #mutation_struct
        #[derive(Default)]
        pub struct #subscription_name;

        struct #default_authorizer_name{}

        struct #default_hook_name{}

        #create_input

        #update_input

        #create_input_transform

        #update_input_transform

        #[async_graphql::Object]
        impl #query_name {
            async fn #list_name(
                &self,
                ctx: &async_graphql::Context<'_>,
                paging: Option<crud::Pagination>,
                filter: Option<#filter_name>,
                sorting: Option<Vec<#sort_name>>,
            ) -> async_graphql::Result<async_graphql::connection::Connection<crud::Cursor, #name>> {
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
                        Box::pin(query
                            .clone()
                            .stream(db)
                            .await?)
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
        #mutation_impl
        #[async_graphql::Subscription]
        impl #subscription_name {
            async fn #stream_name<'ctx>(
                &self,
                ctx: &async_graphql::Context<'ctx>,
                filter: Option<#filter_name>
            ) -> async_graphql::Result<impl Stream<Item=#name>+'ctx>{
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;
                let authorizer=#authorizer_constructor;
                let authorize_condition=crud::Authorizer::authorize(&authorizer,ctx).await?;
                let f = sea_orm::Condition::add(sea_orm::Condition::all(),authorize_condition);
                let f = filter.map_or(f.clone(), |v| f.add(v.build()));
                let query = <sea_orm::Select<#model::Entity> as sea_orm::QueryFilter>::filter(<#model::Entity as sea_orm::EntityTrait>::find(), f);
                Ok(
                    Box::pin(query
                   .clone()
                   .stream(db)
                   .await?)
                   .filter_map(|v|async move {v.ok().map(|v|v.into())})
                    )
            }
        }

        impl From<#model::Model> for #name {
            fn from(_self: #model::Model) -> Self {
                Self {
                    #(#impl_from),*
                }
            }
        }


        impl crud::Authorizer for #default_authorizer_name{}

        impl #default_authorizer_name{
            fn new()->Self{Self{}}
        }

        impl crud::Hook for #default_hook_name{
            type ActiveModel=#model::ActiveModel;
        }

        impl #default_hook_name{
            fn new()->Self{Self{}}
        }
    }
}
