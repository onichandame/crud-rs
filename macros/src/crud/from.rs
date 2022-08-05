use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helper::{get_model, get_struct_fields};

pub fn from_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let model = get_model(input);
    let fields = get_struct_fields(input);
    let impl_from: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .map(|v| {
            let name = v.ident.expect("fields must be named");
            quote! {#name: _self.#name}
        })
        .collect();
    quote! {
        impl From<#model::Model> for #name {
            fn from(_self: #model::Model) -> Self {
                Self {
                    #(#impl_from),*
                }
            }
        }
    }
}
