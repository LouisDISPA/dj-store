#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DeriveInput, Expr, ExprLit, Lit, Meta,
    MetaNameValue, Variant,
};
#[proc_macro_derive(ApiError, attributes(status))]
pub fn api_error(input: TokenStream) -> TokenStream {
    let input = match ::syn::parse::<DeriveInput>(input) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let name = input.ident;
    let Data::Enum(DataEnum { variants, .. }) = input.data else {
    ::core::panicking::panic_fmt(format_args!("ApiError can only be derived for enums"));
};
    let names = variants.iter().map(|variant| &variant.ident);
    let status = variants.iter().map(find_status);
    let docs = variants.iter().map(find_doc);
    let expanded = {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::__private::push_ident(&mut _s, "impl");
        ::quote::__private::push_ident(&mut _s, "axum");
        ::quote::__private::push_colon2(&mut _s);
        ::quote::__private::push_ident(&mut _s, "response");
        ::quote::__private::push_colon2(&mut _s);
        ::quote::__private::push_ident(&mut _s, "IntoResponse");
        ::quote::__private::push_ident(&mut _s, "for");
        ::quote::ToTokens::to_tokens(&name, &mut _s);
        ::quote::__private::push_group(
            &mut _s,
            ::quote::__private::Delimiter::Brace,
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "fn");
                ::quote::__private::push_ident(&mut _s, "into_response");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "self");
                        _s
                    },
                );
                ::quote::__private::push_rarrow(&mut _s);
                ::quote::__private::push_ident(&mut _s, "axum");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "response");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Response");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "match");
                        ::quote::__private::push_ident(&mut _s, "self");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                {
                                    use ::quote::__private::ext::*;
                                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                    #[allow(unused_mut)]
                                    let (mut name, i) = name.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    #[allow(unused_mut)]
                                    let (mut names, i) = names.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    #[allow(unused_mut)]
                                    let (mut status, i) = status.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    #[allow(unused_mut)]
                                    let (mut docs, i) = docs.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    let _: ::quote::__private::HasIterator = has_iter;
                                    while true {
                                        let name = match name.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        let names = match names.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        let status = match status.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        let docs = match docs.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        ::quote::ToTokens::to_tokens(&name, &mut _s);
                                        ::quote::__private::push_colon2(&mut _s);
                                        ::quote::ToTokens::to_tokens(&names, &mut _s);
                                        ::quote::__private::push_fat_arrow(&mut _s);
                                        ::quote::__private::push_group(
                                            &mut _s,
                                            ::quote::__private::Delimiter::Parenthesis,
                                            {
                                                let mut _s = ::quote::__private::TokenStream::new();
                                                ::quote::ToTokens::to_tokens(&status, &mut _s);
                                                ::quote::__private::push_comma(&mut _s);
                                                ::quote::ToTokens::to_tokens(&docs, &mut _s);
                                                _s
                                            },
                                        );
                                        ::quote::__private::push_comma(&mut _s);
                                    }
                                }
                                _s
                            },
                        );
                        ::quote::__private::push_dot(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "into_response");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            ::quote::__private::TokenStream::new(),
                        );
                        _s
                    },
                );
                _s
            },
        );
        _s
    };
    TokenStream::from(expanded)
}
fn find_doc(variant: &Variant) -> String {
    variant
        .attrs
        .iter()
        .find_map(|attr| {
            if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
                if path.is_ident("doc") {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = value {
                        return Some(lit.value().trim().to_string());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| variant.ident.to_string().to_case(Case::Title))
}
fn find_status(variant: &Variant) -> String {
    variant
        .attrs
        .iter()
        .find_map(|attr| {
            if let Meta::List(list) = &attr.meta {
                if list.path.is_ident("status") {
                    if let Ok(Meta::Path(path)) = list.parse_args() {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", path));
                        };
                        return Some(path.get_ident().unwrap().to_string());
                    }
                }
            }
            None
        })
        .expect("Missing #[status(...)] attribute")
}
const _: () = {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    #[used]
    #[allow(deprecated)]
    static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::custom_derive(
            "ApiError",
            &["status"],
            api_error,
        ),
    ];
};
