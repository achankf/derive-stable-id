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
        impl Default for #name {
            fn default() -> Self {
                Self(Default::default())
            }
        }

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

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                use stable_id_traits::Inner;
                self.project().eq(&other.project())
            }
        }

        impl std::cmp::Eq for #name {
            fn assert_receiver_is_total_eq(&self) {
                use stable_id_traits::Inner;
                self.project().assert_receiver_is_total_eq();
            }
        }

        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                use stable_id_traits::Inner;
                self.project().partial_cmp(&other.project())
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                use stable_id_traits::Inner;
                self.project().cmp(&other.project())
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                use stable_id_traits::Inner;
                Self(self.project())
            }
        }

        impl Copy for #name {}

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                use stable_id_traits::Inner;
                self.project().hash(state);
            }
        }
    }
    .into()
}
