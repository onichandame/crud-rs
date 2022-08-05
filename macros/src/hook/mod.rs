use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helper::get_model;

pub fn hook_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let model = get_model(input);
    quote! {impl crud::Hook for #name{
        type ActiveModel=#model ::ActiveModel;
    }}
}
