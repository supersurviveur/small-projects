use parse_more::parse_more_macro_input;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, ExprBlock, Ident};

#[proc_macro]
pub fn make_variant(input: TokenStream) -> TokenStream {
    let args =
        parse_more_macro_input!(input as Punctuated<(Ident, ExprBlock, ExprBlock), syn::Token![,]>);

    args.into_iter()
        .map(|(op_trait, code_unsigned, code_signed)| {
            let op = format_ident!("{}", op_trait.to_string().to_lowercase());
            let op_variant = format_ident!("{op}_variant");
            let op_unsigned = format_ident!("{op}_unsigned");
            let op_signed = format_ident!("{op}_signed");
            quote! {
                #[macro_export(local_inner_macros)]
                macro_rules! #op_variant {
                    ($base:ty, $rhs:ty) => {
                        op_variant! {
                            #op, #op_trait, $base, $rhs
                        }
                    };
                }

                #[macro_export(local_inner_macros)]
                macro_rules! #op_unsigned {
                    ($base:ty, $($from:ty),*) => {
                        $(
                            impl std::ops::#op_trait<$from> for $base {
                                type Output = Self;

                                fn #op(self, rhs: $from) -> Self::Output {
                                    #code_unsigned
                                }
                            }
                            #op_variant! { $base, $from }
                        )*
                    };
                    ($base:ty) => {
                        #op_unsigned! {
                           $base, u8, u16, u32, u64, usize
                        }
                    }
                }

                #[macro_export(local_inner_macros)]
                macro_rules! #op_signed {
                    ($base:ty, $($from:ty),*) => {
                        $(
                            impl std::ops::#op_trait<$from> for $base {
                                type Output = Self;

                                fn #op(self, rhs: $from) -> Self::Output {
                                    #code_signed
                                }
                            }
                            #op_variant! { $base, $from }
                        )*
                    };
                    ($base:ty) => {
                        #op_signed! {
                           $base, i8, i16, i32, i64, isize
                        }
                    }
                }
            }
        })
        .collect::<TokenStream2>()
        .into()
}
