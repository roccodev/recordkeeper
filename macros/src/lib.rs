use proc_macro::TokenStream;

mod save_bin;

#[proc_macro_derive(SaveBin, attributes(loc, assert, size))]
pub fn derive_save_deserialize(item: TokenStream) -> TokenStream {
    save_bin::derive_save_deserialize(item)
}
