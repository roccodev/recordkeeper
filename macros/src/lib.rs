use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, Meta, parse_macro_input};

#[proc_macro_derive(SaveBin, attributes(loc, assert, no_getter))]
pub fn derive_save_deserialize(item: TokenStream) -> TokenStream {
    //let ast = parse_macro_input!(item as DeriveInput);
    //let name = &ast.ident;
    item
}