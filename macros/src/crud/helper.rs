use proc_macro2::{LexError, TokenStream};
use quote::quote;
use syn::{Attribute, Data, Field, Fields, Lit, Meta, MetaList, NestedMeta};

pub fn get_model_name(attrs: &Vec<Attribute>) -> TokenStream {
    get_meta_field(&get_metas(attrs).unwrap(), "model").unwrap()
}

pub fn get_filter_by_type(ty: &str) -> TokenStream {
    match ty {
        "String" => quote! {StringFilter},
        "i32" => quote! {IntFilter},
        "bool" => quote! {BooleanFilter},
        "DateTime" | "chrono::NaiveDateTime" => quote! {NaiveDateFilter},
        _other => {
            panic!("cannot find filter for type {}", ty)
        }
    }
}

pub fn has_meta(meta: &MetaList, path: &str) -> bool {
    meta.nested
        .iter()
        .find_map(|v| {
            if let NestedMeta::Meta(Meta::Path(v)) = v {
                if v.is_ident(path) {
                    Some(true)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map_or(false, |v| v)
}

pub fn get_meta_field(
    meta: &MetaList,
    path: &str,
) -> Result<TokenStream, Box<dyn std::error::Error + Send + Sync>> {
    Ok(meta
        .nested
        .iter()
        .find_map(|v| {
            if let NestedMeta::Meta(Meta::NameValue(v)) = v {
                if v.path.is_ident(path) {
                    if let Lit::Str(v) = &v.lit {
                        Some(v.value())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .ok_or(format!("{} not specified in crud attribute", path))?
        .parse()
        .map_err(|v: LexError| v.to_string())?)
}

pub fn get_meta_list(
    attrs: Vec<Attribute>,
    name: &str,
) -> Result<MetaList, Box<dyn std::error::Error + Send + Sync>> {
    match attrs
        .iter()
        .find_map(|v| {
            if v.path.is_ident(name) {
                Some(v.parse_meta())
            } else {
                None
            }
        })
        .ok_or(format!("helper attribute {} not set", name))??
    {
        Meta::List(v) => Ok(v),
        _other => Err(format!("{} attribute must be a list", name).into()),
    }
}

pub fn get_object_name(
    meta: Vec<Attribute>,
    attr_key: &str,
) -> Result<TokenStream, Box<dyn std::error::Error + Send + Sync>> {
    Ok(get_metas(&meta)
        .and_then(|v| get_meta_field(&v, attr_key))
        .map_err(|v| v.to_string())?)
}

pub fn get_struct_fields(
    data: &Data,
) -> Result<Vec<Field>, Box<dyn std::error::Error + Send + Sync>> {
    if let Data::Struct(v) = &data {
        if let Fields::Named(v) = &v.fields {
            Ok(v.named.iter().map(|v| v.clone()).collect())
        } else {
            Err("fields must be named for struct".into())
        }
    } else {
        Err("only struct can be derived".into())
    }
}

pub fn get_metas(
    attrs: &Vec<Attribute>,
) -> Result<MetaList, Box<dyn std::error::Error + Send + Sync>> {
    get_meta_list(attrs.clone(), "crud")
}
