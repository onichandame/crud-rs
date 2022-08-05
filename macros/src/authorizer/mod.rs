use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn authorizer_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    quote! {
        impl Authorizer for #name {}
    }
}
