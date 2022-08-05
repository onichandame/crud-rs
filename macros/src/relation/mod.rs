use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use self::{connection::get_connections, relation::get_relations};

mod connection;
mod relation;

pub fn relation_expand(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let relations = get_relations(input);
    let connections = get_connections(input);
    let has_complex_fns = relations.len() > 0 || connections.len() > 0;
    if has_complex_fns {
        quote! {
            #[async_graphql::ComplexObject]
            impl #name {
                #(#relations)*

                #(#connections)*
            }
        }
    } else {
        quote! {}
    }
}
