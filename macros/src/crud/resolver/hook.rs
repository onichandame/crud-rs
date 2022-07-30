use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{get_default_hook_name, get_model, has_customized_hook};

pub fn default_hook_expand(input: &DeriveInput) -> TokenStream {
    let default_hook_name = get_default_hook_name(input);
    let model = get_model(input);
    if has_customized_hook(input) {
        quote! {
            struct #default_hook_name{}
            impl Hook for #default_hook_name{
                type ActiveModel=#model::ActiveModel;
            }
            impl #default_hook_name{
                fn new()->Self{Self{}}
            }
        }
    } else {
        quote! {}
    }
}
