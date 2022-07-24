use crud::crud_expand;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod crud;

#[proc_macro_derive(CRUD, attributes(crud))]
pub fn crud_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    crud_expand(input).into()
}
