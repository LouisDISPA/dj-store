use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Attribute, Expr, ExprLit, ItemEnum, Lit, Meta, Path};

// TODO: clean the code
// TODO: add positional arguments ? similar to displaydoc

/// Implements `IntoResponse` for an enum, where each variant has a status code
/// and the response body is the last doc comment or the variant name.
///
/// Attributes:
/// - `status`: the status code for the variant
/// - `default_status`: the default status code for all variants (if no `status` attribute is present)
///
/// # Example
///
/// ```rust
/// # use api_macro::ApiError;
/// # use axum::http::StatusCode;
/// # use axum::response::IntoResponse;
/// #
/// #[derive(ApiError)]
/// #[default_status(StatusCode::INTERNAL_SERVER_ERROR)]
/// pub enum SearchError {
///     /// Room is not found
///     #[status(StatusCode::UNAUTHORIZED)]
///     RoomNotFound,
///     InternalError,
/// }
///
/// let response = SearchError::RoomNotFound.into_response();
/// assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
///
/// let response = SearchError::InternalError.into_response();
/// assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
/// ```
///
#[proc_macro_derive(ApiError, attributes(status, default_status, with_internal_error))]
pub fn api_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let name = input.ident;
    let variants = input.variants;

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

    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #( #name::#names => (#status, #docs).into_response(),)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn find_doc(attributes: &[Attribute]) -> Option<String> {
    for attr in attributes {
        if let Meta::NameValue(name_value) = &attr.meta {
            if name_value.path.is_ident("doc") {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit), ..
                }) = &name_value.value
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

/// Implements `IntoResponse` for an enum, where each variant has a status code
/// and the response body is the last doc comment or the variant name.
///
/// The `error` macro is the same as the derive macro `ApiError`.
///
/// It can be used to add additional variants.
///
/// Attributes:
/// - `internal_error`: add an `InternalError` variant with status code `INTERNAL_SERVER_ERROR`
/// - `unauthorized`: add an `Unauthorized` variant with status code `UNAUTHORIZED`
///
/// # Example
///
/// ```rust
/// # use api_macro::error;
/// # use axum::http::StatusCode;
/// # use axum::response::IntoResponse;
/// #
/// #[error(internal_error unauthorized)]
/// enum SearchError {
///     /// Room is not found
///     #[status(StatusCode::UNAUTHORIZED)]
///     RoomNotFound,
/// }
///
/// let response = SearchError::InternalError.into_response();
/// assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERRO );
///
/// let response = SearchError::Unauthorized.into_response();
/// assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
/// ```
///
#[proc_macro_attribute]
pub fn error(macro_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);

    for token in macro_attrs {
        match token.to_string().as_str() {
            "internal_error" => input.variants.push(parse_quote!(
                #[doc = "Internal error"]
                #[status(axum::http::status::StatusCode::INTERNAL_SERVER_ERROR)]
                InternalError
            )),
            "unauthorized" => input.variants.push(parse_quote!(
                #[doc = "Unauthorized"]
                #[status(axum::http::status::StatusCode::UNAUTHORIZED)]
                Unauthorized
            )),
            token => panic!(
                "Unknown token '{}', possible token: internal_error, unauthorized",
                token
            ),
        }
    }

    input.attrs.insert(
        0,
        parse_quote!(
            #[derive(api_macro::ApiError, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        ),
    );

    TokenStream::from(quote!(#input))
}
