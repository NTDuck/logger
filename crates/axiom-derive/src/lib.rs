extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Erratum)]
pub fn erratum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // In actual implementation, Erratum is supposed to serialize to JSON.
    let expanded = quote! {};

    TokenStream::from(expanded)
}
