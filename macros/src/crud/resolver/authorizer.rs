use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{get_default_authorizer_name, has_customized_authorizer};

pub fn default_authorizer_expand(input: &DeriveInput) -> TokenStream {
    let default_authorizer_name = get_default_authorizer_name(input);
    if has_customized_authorizer(input) {
        quote! {}
    } else {
        quote! {
            struct #default_authorizer_name{}
            impl Authorizer for #default_authorizer_name{}
            impl #default_authorizer_name{
                fn new()->Self{Self{}}
            }
        }
    }
}
