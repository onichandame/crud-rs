use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod filter;
mod helper;
mod resolver;
mod sort;

use crate::crud::{filter::filter_expand, resolver::resolver_expand, sort::sort_expand};

pub fn crud_expand(input: DeriveInput) -> TokenStream {
    let filter = filter_expand(&input);
    let sort = sort_expand(&input);
    let resolver = resolver_expand(&input);
    quote! {
        use crud::futures::prelude::*;

        #filter

        #sort

        #resolver
    }
}
