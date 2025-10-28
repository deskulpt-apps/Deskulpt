#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod event;

use proc_macro::TokenStream;

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    event::proc_derive_event(input)
}
