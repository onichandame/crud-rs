use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Field, Fields, Lit, Meta, MetaList, NestedMeta};

pub fn get_filter_name(input: &DeriveInput) -> TokenStream {
    let metas = get_crud_metas(&input.attrs).unwrap();
    get_value(&metas, "filter").map_or(
        format!("{}Filter", input.ident.to_string())
            .parse()
            .unwrap(),
        |v| v,
    )
}

pub fn get_field_name(input: &DeriveInput) -> TokenStream {
    format!("{}Field", input.ident.to_string()).parse().unwrap()
}

pub fn get_sort_name(input: &DeriveInput) -> TokenStream {
    format!("{}Sort", input.ident.to_string()).parse().unwrap()
}

pub fn get_model(input: &DeriveInput) -> TokenStream {
    let metas = &get_crud_metas(&input.attrs).unwrap();
    get_value(metas, "model").unwrap()
}

pub fn get_filter_by_type(ty: &str) -> TokenStream {
    match ty {
        "String" => quote! {StringFilter},
        "i32" | "i64" | "u32" | "u64" | "isize" | "usize" => quote! {IntFilter},
        "bool" => quote! {BooleanFilter},
        "DateTime" | "chrono :: NaiveDateTime" | "NaiveDateTime" => quote! {DateTimeFilter},
        _other => {
            panic!("cannot find filter for type {}", ty)
        }
    }
}

pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

pub fn get_flag(meta: &MetaList, path: &str) -> bool {
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

pub fn get_value(meta: &MetaList, path: &str) -> Result<TokenStream, String> {
    Ok(meta
        .nested
        .iter()
        .find_map(|v| {
            if let NestedMeta::Meta(Meta::NameValue(v)) = v {
                if v.path.is_ident(path) {
                    if let Lit::Str(v) = &v.lit {
                        Some(v.value().parse().unwrap())
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
        .ok_or(format!("{} not specified in crud attribute", path))?)
}

pub fn get_struct_fields(input: &DeriveInput) -> Vec<Field> {
    if let Data::Struct(v) = &input.data {
        if let Fields::Named(v) = &v.fields {
            v.named.iter().map(|v| v.clone()).collect()
        } else {
            panic!("fields must be named for struct")
        }
    } else {
        panic!("only struct can be derived")
    }
}

pub fn get_crud_metas(attrs: &Vec<Attribute>) -> Option<MetaList> {
    get_meta_lists(attrs, "crud").get(0).map(|v| v.clone())
}

pub fn get_meta_lists(attrs: &Vec<Attribute>, name: &str) -> Vec<MetaList> {
    attrs
        .iter()
        .filter_map(|v| {
            if v.path.is_ident(name) {
                Some(v.parse_meta().unwrap())
            } else {
                None
            }
        })
        .map(|v| match v {
            Meta::List(v) => v,
            _other => panic!("{} attribute must be a list", name),
        })
        .collect::<Vec<_>>()
}

/// str -> "str"
pub fn get_str_literal(ts: &TokenStream) -> TokenStream {
    format!("\"{}\"", ts.to_string()).parse().unwrap()
}
