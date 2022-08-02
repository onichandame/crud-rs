use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{
    get_authorizer_constructor, get_filter_name, get_flag, get_metas, get_model,
};

pub fn subscription_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let metas = get_metas(&input.attrs).unwrap();
    let subscribable = get_flag(&metas, "subscribable");
    let subscription_name = format!("{}Subscription", &name)
        .parse::<TokenStream>()
        .unwrap();
    let stream_name: TokenStream = format!(
        "stream_{}",
        pluralizer::pluralize(&name.to_string(), 2, false)
    )
    .to_lowercase()
    .parse()
    .unwrap();
    let model = get_model(input);
    let filter_name = get_filter_name(input);
    let authorizer_constructor = get_authorizer_constructor(input);
    if subscribable {
        quote! {
            #[derive(Default)]
            pub struct #subscription_name;

            #[async_graphql::Subscription]
            impl #subscription_name {
                async fn #stream_name<'ctx>(
                    &self,
                    ctx: &async_graphql::Context<'ctx>,
                    filter: Option<#filter_name>
                ) -> async_graphql::Result<impl futures::stream::Stream<Item=#name>+'ctx>{
                    use crud::futures::prelude::*;
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
                       .await?
                       .filter_map(|v|async move {v.ok().map(|v|v.into())}))
                    )
                }
            }
        }
    } else {
        quote! {}
    }
}
