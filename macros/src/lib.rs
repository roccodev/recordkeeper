use proc_macro2::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, Field, Ident, Meta};

struct FieldVisitor<'ast> {
    field: &'ast Field,
    location: Option<TokenStream>,
    assert_value: Option<TokenStream>,
}

impl<'ast> FieldVisitor<'ast> {
    fn parser_tokens(&self) -> TokenStream {
        let var_name = &self.field.ident;
        let type_ident = &self.field.ty;

        let loc_code = self.location.as_ref().map(|loc| {
            quote! {
                __IN_BYTES.set_position(#loc.try_into().expect("new #[loc] too large"));
            }
        });

        let assert_code = self.assert_value.as_ref().map(|assert_value| {
            quote! {
                assert_eq!(#assert_value, #var_name);
            }
        });

        let out = quote! {
            #loc_code
            let #var_name = <#type_ident as crate::io::SaveBin>::read(__IN_BYTES.clone())?;
            #assert_code
            __IN_BYTES.set_position(__IN_BYTES.position()
                + <#type_ident as crate::io::SaveBin>::size().try_into().expect("size too large"));
        };
        out.into()
    }

    fn initializer_tokens(&self) -> TokenStream {
        let name = &self.field.ident;
        let out = quote! { #name, };
        out.into()
    }
}

#[proc_macro_derive(SaveBin, attributes(loc, assert))]
pub fn derive_save_deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let item_struct = match item.data {
        Data::Struct(str) => str,
        _ => panic!("SaveBin can only be derived on structs"),
    };

    let field_visitors = item_struct
        .fields
        .iter()
        .map(|f| {
            let mut loc = None;
            let mut assert = None;

            for attr in &f.attrs {
                let path = attr.path();
                let list = match &attr.meta {
                    Meta::List(list) => list,
                    _ => continue,
                };
                if path.is_ident("loc") {
                    loc = Some(list.tokens.clone());
                } else if path.is_ident("assert") {
                    assert = Some(list.tokens.clone());
                }
            }

            FieldVisitor {
                field: f,
                location: loc,
                assert_value: assert,
            }
        })
        .collect::<Vec<_>>();

    let parsers = field_visitors
        .iter()
        .flat_map(|v| v.parser_tokens())
        .collect::<TokenStream>();

    let initializers = field_visitors
        .iter()
        .flat_map(|v| v.initializer_tokens())
        .collect::<TokenStream>();

    let out = quote! {
        impl<'__SRC> #impl_generics crate::io::SaveBin<'__SRC> for #name #ty_generics #where_clause {
            type Error = crate::error::StructError;

            fn read(mut __IN_BYTES: std::io::Cursor<&'__SRC [u8]>) -> Result<Self, Self::Error> {
                // Set up relative positions for start of struct
                let __POS = usize::from(__IN_BYTES.position());
                __IN_BYTES = std::io::Cursor::new(&__IN_BYTES.into_inner()[__POS..]);

                #parsers

                Ok(Self {
                    #initializers
                })
            }
        }
    };

    out.into()
}
