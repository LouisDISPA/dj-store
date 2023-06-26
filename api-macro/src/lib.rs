use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse, parse_macro_input, parse_quote, punctuated::Punctuated, Attribute, Ident, ItemEnum,
    Path, Token,
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
/// #[derive(ApiError, Debug)]
/// #[default_status(StatusCode::INTERNAL_SERVER_ERROR)]
/// pub enum SearchError {
///     #[status(StatusCode::UNAUTHORIZED)]
///     RoomNotFound,
///     InternalError,
/// }
///
/// // Implement the display trait for the enum
/// // (this is required by the derive macro)
/// impl std::fmt::Display for SearchError {
///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///       match self {
///          SearchError::RoomNotFound => write!(f, "Room not found"),
///          SearchError::InternalError => write!(f, "Internal error"),
///      }
///   }
/// }
///
/// let response = SearchError::RoomNotFound.into_response();
/// assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
///
/// let response = SearchError::InternalError.into_response();
/// assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
/// ```
///
#[proc_macro_derive(ApiError, attributes(status, default_status))]
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
                .or_else(|| add_error(syn::Error::new(
                    variant.ident.span(),
                    "Missing #[status(...)] attribute, or #[default_status(...)] attribute on the enum.",
                )))
        })
        .collect();

    if !errors.is_empty() {
        return errors
            .into_iter()
            .map(syn::Error::into_compile_error)
            .map(TokenStream::from)
            .collect();
    }

    // Assert that the enum type implements Display. If not, user sees an error
    let assert_display = quote_spanned! {name.span()=>
        struct _AssertDisplay where #name: std::fmt::Display;
    };

    // Assert that the enum type implements Debug. If not, user sees an error
    let assert_debug = quote_spanned! {name.span()=>
        struct _AssertDebug where #name: std::fmt::Debug;
    };

    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                #assert_display
                #assert_debug
                log::error!("Error: {:?}", self);
                let status = self.status();
                let body = self.to_string();
                (status, body).into_response()
            }
        }

        impl #name {
            /// Returns the status code for this error.
            pub fn status(&self) -> axum::http::StatusCode {
                match self {
                    #(#name::#names {..} => #status,)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
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
/// - `internal_error`: add an `InternalError(sea_orm::DbErr)` variant with status code `INTERNAL_SERVER_ERROR`
/// - `unauthorized`: add an `Unauthorized` variant with status code `UNAUTHORIZED`
///
/// # Example
///
/// ```rust
/// # use api_macro::error;
/// # use axum::http::StatusCode;
/// # use axum::response::IntoResponse;
/// #
/// #[error(unauthorized)]
/// enum SearchError {
///     /// Room is not found
///     #[status(StatusCode::UNAUTHORIZED)]
///     RoomNotFound,
/// }
///
/// let response = SearchError::RoomNotFound.into_response();
/// assert_eq!(response.status(), StatusCode::UNAUTHORIZED );
///
/// let response = SearchError::Unauthorized.into_response();
/// assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
/// ```
///
#[proc_macro_attribute]
pub fn error(macro_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let macro_attrs = parse_macro_input!(macro_attrs as Args);
    let mut input = parse_macro_input!(input as ItemEnum);

    let new_variants = macro_attrs.variants.iter().map(ErrorVariant::to_variant);
    input.variants.extend(new_variants);

    let derive = parse_quote!(
        #[derive(api_macro::ApiError, thiserror::Error, displaydoc::Display, Debug)]
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
                InternalError(
                    #[from]
                    sea_orm::DbErr
                )
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
