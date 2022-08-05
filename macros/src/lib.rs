use authorizer::authorizer_expand;
use crud::crud_expand;
use hook::hook_expand;
use proc_macro::TokenStream;
use relation::relation_expand;
use syn::{parse_macro_input, DeriveInput};

mod authorizer;
mod crud;
mod helper;
mod hook;
mod relation;

#[proc_macro_derive(CRUD, attributes(crud))]
pub fn crud_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    crud_expand(&input).into()
}

#[proc_macro_derive(Authorizer, attributes(crud))]
pub fn authorizer_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    authorizer_expand(&input).into()
}

#[proc_macro_derive(Hook, attributes(crud))]
pub fn hook_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    hook_expand(&input).into()
}

#[proc_macro_derive(Relation, attributes(relation, connection))]
pub fn relation_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    relation_expand(&input).into()
}
