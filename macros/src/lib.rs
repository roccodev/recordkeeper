use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, Meta, parse_macro_input};

#[proc_macro_attribute]
pub fn loc(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let loc = parse_macro_input!(input as Attribute);
    let loc: Ident = loc.parse_args().expect("failed to parse loc args");
    quote! {

    }
}

#[proc_macro_derive(SaveBin, attributes(loc))]
pub fn derive_save_deserialize(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;
}