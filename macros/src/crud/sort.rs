use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{get_meta_field, get_metas, get_object_name, get_struct_fields};

pub fn sort_expand(input: &DeriveInput) -> TokenStream {
    let field_name = get_object_name(input.attrs.clone(), "field").map_or(
        format!("{}Field", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let sort_name = get_object_name(input.attrs.clone(), "sort").map_or(
        format!("{}Sort", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let model = get_meta_field(&get_metas(&input.attrs).unwrap(), "model").unwrap();
    let fields = get_struct_fields(&input.data).unwrap();
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
            Some((quote! {#col}, quote! {Self::#col=>#model ::Column:: #col}))
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
            direction: crud::SortDirection,
        }

        impl Into<#model ::Column> for #field_name{
            fn into(self)->#model ::Column{
                match self{
                    #(#into_body),*
                }
            }
        }

        impl #sort_name{
            pub fn apply_sort<TQuery: sea_orm::QueryOrder>(&self, query: TQuery)->TQuery{
                self.direction.apply_sort(query,Into::<#model ::Column>::into(self.field.clone()))
            }
        }
    }
}
