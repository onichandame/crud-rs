use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use self::{
    authorizer::default_authorizer_expand, hook::default_hook_expand, mutation::mutation_expand,
    query::query_expand, subscription::subscription_expand,
};

mod authorizer;
mod hook;
mod mutation;
mod query;
mod subscription;

pub fn resolver_expand(input: &DeriveInput) -> TokenStream {
    let default_authorizer = default_authorizer_expand(input);
    let default_hook = default_hook_expand(input);
    let query = query_expand(input);
    let mutation = mutation_expand(input);
    let subscription = subscription_expand(input);
    quote! {
        #default_authorizer
        #default_hook
        #query
        //#mutation
        //#subscription


    }
}
