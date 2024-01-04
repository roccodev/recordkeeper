use proc_macro2::TokenStream;

use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Field, Meta, Token};

struct FieldVisitor<'ast> {
    field: &'ast Field,
    location: Option<TokenStream>,
    assert_value: Option<TokenStream>,
    assert_error: Option<TokenStream>,
}

impl<'ast> FieldVisitor<'ast> {
    fn parser_tokens(&self) -> TokenStream {
        let var_name = &self.field.ident;
        let type_ident = &self.field.ty;

        let loc_code = self.location.as_ref().map(|loc| {
            quote! {   
                #[cfg(debug_assertions)]
                {
                    let current = __IN_BYTES.stream_position()? - __BASE_OFFSET;
                    if #loc < current {
                        panic!("New location 0x{:x} is lower than current location 0x{:x} for field {}",
                            #loc, current, stringify!(#var_name));
                    }
                }
                __IN_BYTES.seek(std::io::SeekFrom::Start(__BASE_OFFSET + #loc))?;  
            }
        });

        let assert_error = self.assert_error.clone().unwrap_or_else(|| {
            quote! {
                crate::error::SaveError::AssertionError(format!("(Actual) {:?} != (Expected) {:?}",
                    ACTUAL, EXPECTED))
            }
        });

        let assert_code = self.assert_value.as_ref().map(|assert_value| {
            let field_type = self.field.ty.to_token_stream();
            quote! {
                let EXPECTED: #field_type = #assert_value;
                let ACTUAL = #var_name;
                if EXPECTED != ACTUAL {
                    return Err(#assert_error)
                }
            }
        });

        quote! {
            #loc_code
            let __SIZE = <#type_ident as crate::io::SaveBin>::size();
            let __POS_BEFORE_READ = __IN_BYTES.stream_position()?;
            let #var_name = <#type_ident as crate::io::SaveBin>::read(__IN_BYTES)?;
            __IN_BYTES.seek(std::io::SeekFrom::Start(__POS_BEFORE_READ + __SIZE as u64))?;
            #assert_code
        }
    }

    fn writer_tokens(&self) -> TokenStream {
        let name = &self.field.ident;
        let field_type = self.field.ty.to_token_stream();

        let loc_code = self.location.as_ref().map(|loc| {
            quote! {
                __POS = #loc;
            }
        });

        quote! {
            #loc_code
            let __TMP_BYTES = &mut __OUT_BYTES[__POS..];
            self. #name .write(__TMP_BYTES)?;
            __POS += <#field_type as crate::io::SaveBin>::size();
        }
    }

    fn size_calc_tokens(&self) -> TokenStream {
        let type_ident = &self.field.ty;
        let field_name = self.field.ident.to_token_stream();

        match &self.location {
            Some(loc) => quote! {
                #[cfg(debug_assertions)]
                if #loc < current_loc {
                    panic!("New location 0x{:x} is lower than current location 0x{:x} for field {}",
                        #loc, current_loc, stringify!(#field_name));
                }
                let _size = <#type_ident as crate::io::SaveBin>::size();
                size += _size + #loc - current_loc;
                current_loc = #loc + _size;
            },
            None => quote! {
                let _size = <#type_ident as crate::io::SaveBin>::size();
                size += _size;
                current_loc += _size;
            },
        }
    }
}

#[proc_macro_derive(SaveBin, attributes(loc, assert, size))]
pub fn derive_save_deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    let name = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let item_struct = match item.data {
        Data::Struct(str) => str,
        _ => panic!("SaveBin can only be derived on structs"),
    };

    let expected_size = item
        .attrs
        .iter()
        .find(|a| a.path().is_ident("size"))
        .map(|a| match &a.meta {
            Meta::List(l) => l.tokens.clone(),
            _ => panic!("syntax: #[size(N)]"),
        });

    let field_visitors = item_struct
        .fields
        .iter()
        .map(|f| {
            let mut loc = None;
            let mut assert = None;
            let mut assert_error = None;

            for attr in &f.attrs {
                let path = attr.path();
                let list = match &attr.meta {
                    Meta::List(list) => list,
                    _ => continue,
                };
                if path.is_ident("loc") {
                    loc = Some(list.tokens.clone());
                } else if path.is_ident("assert") {
                    let parts: Punctuated<Expr, Token!(,)> = list.parse_args_with(Punctuated::parse_terminated)
                    .expect(
                        "syntax: #[assert(EXPECTED_VALUE)], or #[assert(EXPECTED, custom_error)",
                    );
                    let mut parts = parts.into_iter();
                    assert = Some(parts.next().unwrap().into_token_stream());
                    assert_error = parts.next().map(ToTokens::into_token_stream);
                }
            }

            FieldVisitor {
                field: f,
                location: loc,
                assert_value: assert,
                assert_error,
            }
        })
        .collect::<Vec<_>>();

    let parsers = field_visitors
        .iter()
        .flat_map(|v| v.parser_tokens())
        .collect::<TokenStream>();

    let writers = field_visitors
        .iter()
        .flat_map(|v| v.writer_tokens())
        .collect::<TokenStream>();

    let size_calc = field_visitors
        .iter()
        .flat_map(|v| v.size_calc_tokens())
        .collect::<TokenStream>();

    let extra_size = expected_size.map(|size| {
        quote! {
            #[cfg(debug_assertions)]
            if size > #size {
                panic!("Struct {} too large, can't add padding. Expected max {} bytes, found {}.",
                    stringify!(#name), #size, size);
            }
            size = #size;
        }
    });

    let field_names: Vec<_> = field_visitors
        .iter()
        .map(|v| v.field.ident.as_ref().unwrap())
        .collect();

    let out = quote! {
        impl #impl_generics crate::io::SaveBin for #name #ty_generics #where_clause {
            type ReadError = crate::error::SaveError;
            type WriteError = crate::error::SaveError;

            fn read(__IN_BYTES: &mut std::io::Cursor::<&[u8]>) -> Result<Self, Self::ReadError> {
                use std::io::Seek;
                // Set up relative positions for start of struct.
                let __BASE_OFFSET = __IN_BYTES.stream_position()?;

                #parsers

                let __SIZE = <Self as crate::io::SaveBin>::size();
                __IN_BYTES.seek(std::io::SeekFrom::Start(__BASE_OFFSET + __SIZE as u64))?;

                Ok(Self { #(#field_names),*})
            }

            fn write(&self, mut __OUT_BYTES: &mut [u8]) -> Result<(), Self::WriteError> {
                let mut __POS = 0;
                #writers
                Ok(())
            }

            fn size() -> usize { // TODO: const?
                let mut current_loc: usize = 0;
                let mut size: usize = 0;

                #size_calc
                #extra_size

                size
            }
        }
    };

    out.into()
}
