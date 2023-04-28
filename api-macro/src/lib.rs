use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DeriveInput, Expr, ExprLit, Lit, Meta, MetaNameValue, Path,
    Variant,
};

#[proc_macro_derive(ApiError, attributes(status))]
pub fn api_error(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let Data::Enum(DataEnum { variants, ..}) = input.data else {
        panic!("ApiError can only be derived for enums");
    };

    let names = variants.iter().map(|variant| &variant.ident);
    let status = variants.iter().map(find_status);
    let docs = variants.iter().map(find_doc);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #( #name::#names => (#status, #docs),)*
                }.into_response()
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn find_doc(variant: &Variant) -> String {
    variant
        .attrs
        .iter()
        .find_map(|attr| {
            if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
                if path.is_ident("doc") {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) = value
                    {
                        return Some(lit.value().trim().to_string());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| variant.ident.to_string().to_case(Case::Title))
}

fn find_status(variant: &Variant) -> Path {
    variant
        .attrs
        .iter()
        .find_map(|attr| {
            if let Meta::List(list) = &attr.meta {
                if list.path.is_ident("status") {
                    if let Ok(Meta::Path(path)) = list.parse_args() {
                        return Some(path);
                    }
                }
            }
            None
        })
        .expect("Missing #[status(...)] attribute")
}
