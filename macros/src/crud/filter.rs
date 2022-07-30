use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Type};

use crate::crud::helper::get_struct_fields;

use super::helper::{get_filter_by_type, get_filter_name, get_model};

pub fn filter_expand(input: &DeriveInput) -> TokenStream {
    let filter_name = get_filter_name(input);
    let model = get_model(input);
    let fields = get_struct_fields(input);
    let (filter_body, build_body): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            let name = v.ident.expect("fields must be named");
            let col: TokenStream = format!(
                "{}::Column::{}",
                model,
                name.to_string().to_case(Case::Pascal)
            )
            .parse()
            .unwrap();
            let ty = match v.ty {
                Type::Path(path) => path.path.to_token_stream(),
                _other => {
                    panic!("type not supported")
                }
            };
            let filter = get_filter_by_type(ty.to_string().as_str());
            Some((
                quote! {
                    #name: Option<#filter>
                },
                quote! {
                if let Some(v)=&self.#name{
                    filter=filter.add(v.build(#col));
                }
                },
            ))
        })
        .unzip();
    if filter_body.len() > 0 {
        quote! {
            #[derive(async_graphql::InputObject, Default, Debug)]
            pub struct #filter_name {
                #(#filter_body),*
            }

            impl #filter_name {
                pub fn build(&self)->sea_orm::Condition{
                    let mut filter = sea_orm::Condition::all();
                    #(#build_body)*
                    filter
                }
            }
        }
    } else {
        quote! {}
    }
}
