use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::get_struct_fields;

use super::helper::{get_field_name, get_model, get_sort_name};

pub fn sort_expand(input: &DeriveInput) -> TokenStream {
    let field_name = get_field_name(input);
    let sort_name = get_sort_name(input);
    let model_name = get_model(input);
    let fields = get_struct_fields(input);
    let (fields_body, into_body): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            let col: TokenStream = v
                .ident
                .expect("fields must be named")
                .to_string()
                .to_case(Case::Pascal)
                .parse()
                .unwrap();
            Some((
                quote! {#col},
                quote! {Self::#col=>#model_name ::Column:: #col},
            ))
        })
        .unzip();
    quote! {
        #[derive(async_graphql::Enum, Clone, Copy, Eq, PartialEq)]
        pub enum #field_name {
            #(#fields_body),*
        }

        #[derive(async_graphql::InputObject)]
        pub struct #sort_name {
            field: #field_name,
            direction: SortDirection,
        }

        impl Into<#model_name ::Column> for #field_name{
            fn into(self)->#model_name ::Column{
                match self{
                    #(#into_body),*
                }
            }
        }

        impl #sort_name{
            pub fn apply_sort<TQuery: sea_orm::QueryOrder>(&self, query: TQuery)->TQuery{
                self.direction.apply_sort(query,Into::<#model_name ::Column>::into(self.field.clone()))
            }
        }
    }
}
