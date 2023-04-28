use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Expr, ExprLit, Lit, Meta,
    MetaNameValue, Path,
};

#[proc_macro_derive(ApiError, attributes(status, default_status, with_internal_error))]
pub fn api_error(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let Data::Enum(DataEnum { variants, ..}) = input.data else {
        panic!("ApiError can only be derived for enums");
    };

    let default_status = find_status(&input.attrs, "default_status");

    let names = variants.iter().map(|variant| &variant.ident);
    let status = variants.iter().map(|variant| {
        find_status(&variant.attrs, "status")
            .or_else(|| default_status.clone())
            .expect("Missing #[status(...)] attribute")
    });
    let docs = variants.iter().map(|variant| {
        find_doc(&variant.attrs).unwrap_or_else(|| variant.ident.to_string().to_case(Case::Title))
    });

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

fn find_doc(attributes: &[Attribute]) -> Option<String> {
    for attr in attributes {
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
    }
    None
}

fn find_status(attributes: &[Attribute], ident: &str) -> Option<Path> {
    for attr in attributes {
        if let Meta::List(list) = &attr.meta {
            if list.path.is_ident(ident) {
                if let Ok(Meta::Path(path)) = list.parse_args() {
                    return Some(path);
                }
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn error(macro_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = input;

    let Data::Enum(DataEnum { variants, ..}) = data else {
        panic!("ApiError can only be derived for enums");
    };

    let mut internal_error = Some(quote!(
        // Internal error.
        #[status(axum::http::status::StatusCode::INTERNAL_SERVER_ERROR)]
        InternalError,
    ));

    if macro_attrs
        .into_iter()
        .find(|test| &test.to_string() == "no_internal_error")
        .is_some()
    {
        internal_error = None
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #[derive(api_macro::ApiError, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #(#attrs)*
        #vis enum #ident #generics {
            #variants
            #internal_error
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
