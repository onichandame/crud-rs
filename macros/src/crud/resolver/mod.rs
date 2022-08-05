use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use self::{mutation::mutation_expand, query::query_expand, subscription::subscription_expand};

mod mutation;
mod query;
mod subscription;

pub fn resolver_expand(input: &DeriveInput) -> TokenStream {
    let query = query_expand(input);
    let mutation = mutation_expand(input);
    let subscription = subscription_expand(input);
    quote! {
        #query
        #mutation
        #subscription


    }
}
