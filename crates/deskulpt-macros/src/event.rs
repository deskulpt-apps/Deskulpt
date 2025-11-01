//! Internals for the `Event` derive macro.

use heck::ToKebabCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, parse_macro_input};

/// Token stream processor for the `Event` derive macro.
///
/// This implements the `deskulpt_common::event::Event` trait for the struct,
/// setting the `NAME` constant to the struct name without the `Event` suffix,
/// converted to kebab-case. If the struct name does not end with `Event` or is
/// just `Event`, a compilation error is returned.
pub fn proc_derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let name = ident.to_string();
    let Some(name) = name.strip_suffix("Event") else {
        return syn::Error::new_spanned(
            &ident,
            format!("The name of an event must end with 'Event'; got '{name}'"),
        )
        .to_compile_error()
        .into();
    };
    if name.is_empty() {
        return syn::Error::new_spanned(&ident, "The name of an event cannot be just 'Event'")
            .to_compile_error()
            .into();
    }
    let name = name.to_kebab_case();
    let lit = LitStr::new(&name, ident.span());

    let expanded = quote! {
        impl #impl_generics ::deskulpt_common::event::Event for #ident #ty_generics #where_clause {
            const NAME: &'static str = concat!(env!("CARGO_PKG_NAME"), "://", #lit);
        }
    };
    TokenStream::from(expanded)
}
