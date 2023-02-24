use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StableId)]
/**
Derives all traits introduced by the `stable-id-traits` crate.
The struct should be a tuple which contains an unsigned numeric primitive type.
*/
pub fn derive_stable_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let id_data_type = match input.data {
        syn::Data::Struct(ref data) => {
            let len = data.fields.len();
            assert_eq!(len, 1, "derive struct should have 1 field");
            let field = data.fields.iter().next().unwrap();

            if let syn::Type::Path(ref tp) = field.ty {
                let segments = &tp.path.segments;
                assert_eq!(len, 1, "should have 1 segment");
                &segments[0].ident
            } else {
                unreachable!("expecting a Path")
            }
        }
        _ => unreachable!("expecting a Struct"),
    }
    .clone();

    quote! {
        impl stable_id_traits::Successor for #name {
            fn next_value(self) -> Self {
                assert!(self != stable_id_traits::Maximum::max_value());
                let Self(value) = self;
                Self(value.next_value())
            }
        }

        impl stable_id_traits::Predecessor for #name {
            fn prev_value(self) -> Self {
                assert!(self != Default::default());
                let Self(value) = self;
                Self(value.prev_value())
            }
        }


        impl stable_id_traits::Maximum for #name {
            fn max_value() -> Self {
                Self(#id_data_type::max_value())
            }
        }

        impl stable_id_traits::CastUsize for #name {
            fn cast_from(val: usize) -> Self {
                let val = #id_data_type::cast_from(val);
                Self(val as #id_data_type)
            }

            fn cast_to(self) -> usize {
                self.0 as usize
            }
        }

        impl stable_id_traits::Inner<#id_data_type> for #name {
            fn project(self) -> #id_data_type {
                self.0
            }
        }
    }
    .into()
}
