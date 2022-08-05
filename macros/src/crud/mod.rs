use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod filter;
mod from;
mod helper;
mod resolver;
mod sort;

use crate::crud::{filter::filter_expand, resolver::resolver_expand, sort::sort_expand};

use self::from::from_expand;

pub fn crud_expand(input: &DeriveInput) -> TokenStream {
    let filter = filter_expand(input);
    let sort = sort_expand(input);
    let from = from_expand(input);
    let resolver = resolver_expand(input);
    quote! {
        #filter
        #sort
        #from
        #resolver
    }
}
