use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse, parse_macro_input, parse_quote, punctuated::Punctuated, Attribute, Expr, ExprLit, Ident,
    ItemEnum, Lit, Meta, Path, Token,
};

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

    let mut errors = vec![];

    let mut add_error = |err: syn::Error| {
        errors.push(err);
        None
    };

    let default_status = find_status(&input.attrs, "default_status").unwrap_or_else(&mut add_error);

    let names = variants.iter().map(|variant| &variant.ident);
    let status: Vec<Path> = variants
        .iter()
        .flat_map(|variant| {
            find_status(&variant.attrs, "status")
                .unwrap_or_else(&mut add_error)
                .or_else(|| default_status.clone())
                .or_else(||
                    add_error(syn::Error::new(
                        variant.ident.span(),
                        "Missing #[status(...)] attribute, or #[default_status(...)] attribute on the enum.",
                    ))
                )
        })
        .collect();
    let docs = variants.iter().map(|variant| {
        find_doc(&variant.attrs).unwrap_or_else(|| variant.ident.to_string().to_case(Case::Title))
    });

    if !errors.is_empty() {
        return errors
            .into_iter()
            .map(syn::Error::into_compile_error)
            .map(TokenStream::from)
            .collect();
    }

    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #(#name::#names => {
                        log::warn!(#docs);
                        (#status, #docs).into_response()
                    },)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Finds the last doc comment in the attributes.
fn find_doc(attributes: &[Attribute]) -> Option<String> {
    for attr in attributes.iter().rev() {
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

/// Finds the status code in the attributes.
fn find_status(attributes: &[Attribute], ident: &str) -> Result<Option<Path>, syn::Error> {
    for attr in attributes {
        if attr.path().is_ident(ident) {
            return Ok(Some(attr.parse_args()?));
        }
    }
    Ok(None)
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
    let macro_attrs = parse_macro_input!(macro_attrs as Args);
    let mut input = parse_macro_input!(input as ItemEnum);

    input
        .variants
        .extend(macro_attrs.variants.iter().map(ErrorVariant::to_variant));

    let derive = parse_quote!(
        #[derive(api_macro::ApiError, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    );

    input.attrs.insert(0, derive);

    TokenStream::from(quote!(#input))
}

/// The arguments for the `error` macro.
struct Args {
    variants: Punctuated<ErrorVariant, Token![,]>,
}

impl parse::Parse for Args {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        Ok(Self {
            variants: Punctuated::parse_terminated(input)?,
        })
    }
}

/// The variants that can be added with the `error` macro.
enum ErrorVariant {
    InternalError,
    Unauthorized,
}

impl ErrorVariant {
    fn to_variant(&self) -> syn::Variant {
        match self {
            ErrorVariant::InternalError => parse_quote!(
                #[doc = "Internal error"]
                #[status(axum::http::status::StatusCode::INTERNAL_SERVER_ERROR)]
                InternalError
            ),
            ErrorVariant::Unauthorized => parse_quote!(
                #[doc = "Unauthorized"]
                #[status(axum::http::status::StatusCode::UNAUTHORIZED)]
                Unauthorized
            ),
        }
    }
}

impl parse::Parse for ErrorVariant {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let ident = input.parse::<Ident>()?;
        // is there a cleaner way to do this?
        match ident.to_string().as_str() {
            "internal_error" => Ok(ErrorVariant::InternalError),
            "unauthorized" => Ok(ErrorVariant::Unauthorized),
            _ => Err(parse::Error::new(
                ident.span(),
                "Unknown identifyer, possible: internal_error, unauthorized",
            )),
        }
    }
}
