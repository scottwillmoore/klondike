use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = match input.data {
        Data::Enum(data_enum) => {
            let name = input.ident;

            let mut length: usize = 0;

            let mut from_match_arms = quote! {};
            let mut into_match_arms = quote! {};

            for variant in data_enum.variants {
                match variant.fields {
                    Fields::Unit => {
                        let variant = variant.ident;

                        from_match_arms = quote! {
                            #from_match_arms
                            #length => ::core::option::Option::Some(Self::#variant),
                        };

                        into_match_arms = quote! {
                            #into_match_arms
                            Self::#variant => #length,
                        };

                        length += 1;
                    }
                    _ => todo!(),
                }
            }

            if length > 0 {
                quote! {
                    #[automatically_derived]
                    impl ::enum_trait::Enum for #name {
                        const LENGTH: ::core::primitive::usize = #length;

                        fn from_index(index: ::core::primitive::usize) -> Option<Self> {
                            match index {
                                #from_match_arms
                                _ => ::core::option::Option::None,
                            }
                        }

                        fn to_index(self) -> ::core::primitive::usize {
                            match self {
                                #into_match_arms
                            }
                        }
                    }
                }
            } else {
                todo!()
            }
        }
        _ => todo!(),
    };

    output.into()
}
