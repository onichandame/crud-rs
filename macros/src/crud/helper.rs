use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Field, Fields, Lit, Meta, MetaList, NestedMeta};

pub fn get_default_hook_name(input: &DeriveInput) -> TokenStream {
    format!("_DefaultHook{}", input.ident.to_string())
        .parse()
        .unwrap()
}

pub fn get_default_hook_constructor(input: &DeriveInput) -> TokenStream {
    format!("{}::new()", get_default_hook_name(input))
        .parse()
        .unwrap()
}

pub fn has_customized_hook(input: &DeriveInput) -> bool {
    let metas = get_metas(&input.attrs).unwrap();
    get_value(&metas, "hook").map_or(false, |_| true)
}

pub fn get_hook_constructor(input: &DeriveInput) -> TokenStream {
    let metas = get_metas(&input.attrs).unwrap();
    get_value(&metas, "hook").map_or(get_default_hook_constructor(input), |v| v)
}

pub fn get_default_authorizer_name(input: &DeriveInput) -> TokenStream {
    format!("_DefaultAuthorizer{}", input.ident.to_string())
        .parse()
        .unwrap()
}

pub fn get_default_authorizer_constructor(input: &DeriveInput) -> TokenStream {
    format!("{}::new()", get_default_authorizer_name(input).to_string())
        .parse()
        .unwrap()
}

pub fn has_customized_authorizer(input: &DeriveInput) -> bool {
    let metas = get_metas(&input.attrs).unwrap();
    get_value(&metas, "authorizer").map_or(false, |_| true)
}

pub fn get_authorizer_constructor(input: &DeriveInput) -> TokenStream {
    let metas = get_metas(&input.attrs).unwrap();
    get_value(&metas, "authorizer").map_or(get_default_authorizer_constructor(input), |v| v)
}

pub fn get_filter_name(input: &DeriveInput) -> TokenStream {
    let metas = get_metas(&input.attrs).unwrap();
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
    let metas = &get_metas(&input.attrs).unwrap();
    get_value(metas, "model").unwrap()
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

pub fn get_metas(attrs: &Vec<Attribute>) -> Option<MetaList> {
    get_meta_list(attrs, "crud")
}

fn get_meta_list(attrs: &Vec<Attribute>, name: &str) -> Option<MetaList> {
    attrs
        .iter()
        .find_map(|v| {
            if v.path.is_ident(name) {
                Some(v.parse_meta().unwrap())
            } else {
                None
            }
        })
        .and_then(|v| match v {
            Meta::List(v) => Some(v),
            _other => panic!("{} attribute must be a list", name),
        })
}
