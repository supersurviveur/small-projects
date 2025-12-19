mod generics;
mod where_clauses;

use std::iter::repeat;

use parse_more::{
    filler, parse_more, parse_more_macro_input, Braced, Bracketed, Concat, Parenthesized,
    ParseMore, ParseMoreWrapper,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, visit::Visit, visit_mut::VisitMut, Error, Ident, LitStr,
    Token, Type, TypePath, TypeTuple, WhereClause,
};

use crate::{
    generics::{GenericsExtractor, InferReplaceType, ReplaceInferedTypes},
    where_clauses::ComputeWhereClauses,
};

fn crate_prefix(in_crate: bool) -> TokenStream2 {
    if in_crate {
        quote! {crate}
    } else {
        quote! {::functional_type}
    }
}

#[proc_macro]
pub fn generate_unsigned_integers(input: TokenStream) -> TokenStream {
    let n = parse_more_macro_input!(input as u64);
    let res: TokenStream2 = (0..=n)
        .map(|i| {
            let name = format_ident!("U{i}");
            let r#type = generate_unsigned_integer_impl(i, true);
            quote! {
                pub type #name = #r#type;
            }
        })
        .collect();
    res.into()
}
fn generate_unsigned_integer_impl(i: u64, in_crate: bool) -> TokenStream2 {
    let crate_name = crate_prefix(in_crate);

    if i == 0 {
        return quote!(#crate_name::integer::unsigned::UIntDelimiter);
    }

    let bits = (0..=i.ilog2()).rev().map(|b| {
        if (i >> b) & 1 == 0 {
            quote!(#crate_name::bool::False)
        } else {
            quote!(#crate_name::bool::True)
        }
    });

    bits.fold(
        quote!(#crate_name::integer::unsigned::UIntDelimiter),
        |acc, bit| quote!(#crate_name::integer::unsigned::UInt<#acc, #bit>),
    )
}
#[proc_macro]
pub fn generate_unsigned_integer(input: TokenStream) -> TokenStream {
    generate_unsigned_integer_impl(parse_more_macro_input!(input as u64), false).into()
}

#[proc_macro]
pub fn generate_signed_integers(input: TokenStream) -> TokenStream {
    let n = parse_more_macro_input!(input as i64);
    let res: TokenStream2 = (-n..=n)
        .map(|i| {
            let letter = if i.is_negative() { "N" } else { "P" };
            let name = format_ident!("{letter}{}", i.abs() as u64);
            let r#type = generate_signed_integer_impl(i, true);
            quote! {
                pub type #name = #r#type;
            }
        })
        .collect();
    res.into()
}
fn generate_signed_integer_impl(i: i64, in_crate: bool) -> TokenStream2 {
    let crate_name = crate_prefix(in_crate);

    let unsigned = format_ident!("U{}", i.abs() as u64);
    let sign = if i.is_negative() {
        quote! {#crate_name::bool::True}
    } else {
        quote! {#crate_name::bool::False}
    };

    quote!(#crate_name::integer::signed::SInt<#sign, #crate_name::integer::unsigned::#unsigned>)
}
#[proc_macro]
pub fn generate_signed_integer(input: TokenStream) -> TokenStream {
    generate_signed_integer_impl(parse_more_macro_input!(input as i64), false).into()
}

#[parse_more]
#[derive(Clone)]
enum OperandType {
    Path(TypePath),
    Tuple(TypeTuple),
}

impl OperandType {
    fn get_type(self) -> Type {
        match self {
            OperandType::Path(type_path) => Type::Path(type_path),
            OperandType::Tuple(type_tuple) => Type::Tuple(type_tuple),
        }
    }
}

#[derive(Clone)]
struct DocInner {
    pub doc: LitStr,
}

impl ParseMore for DocInner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let doc_attr = input.parse::<syn::Ident>()?;
        if doc_attr.to_string() != "doc" {
            return Err(Error::new(doc_attr.span(), "'doc' attribute expected"));
        }
        input.parse::<Token![=]>()?;
        let doc = input.parse()?;
        Ok(Self { doc })
    }
}

#[parse_more]
#[derive(Clone)]
struct DocAttribute {
    pub _pound_token: Token![#],
    pub inner: Bracketed<DocInner>,
}
#[parse_more]
#[derive(Clone)]
struct DocAttributes {
    pub attrs: Vec<DocAttribute>,
}
trait ToDocAttr {
    fn to_doc_attr(&self) -> TokenStream2;
}
impl ToDocAttr for DocAttributes {
    fn to_doc_attr(&self) -> TokenStream2 {
        self.attrs
            .iter()
            .map(|attr| {
                let doc = attr.inner.0.doc.value();
                let doc = doc.trim();
                quote! {#[doc = #doc]}
            })
            .collect()
    }
}
impl ToDocAttr for DocAttribute {
    fn to_doc_attr(&self) -> TokenStream2 {
        let doc = self.inner.0.doc.value();
        let doc = doc.trim_start();
        quote! {#[doc = #doc]}
    }
}

impl ToDocAttr for Option<DocAttributes> {
    fn to_doc_attr(&self) -> TokenStream2 {
        self.as_ref().map_or(quote! {}, DocAttributes::to_doc_attr)
    }
}

type OperationLine<Op> = Concat<
    Option<DocAttributes>,
    ArgsMatch<Concat<OperandType, Op, Type>>,
    Token![=],
    OperandType,
    Option<WhereClause>,
>;

#[parse_more]
struct OperationsGroup {
    #[filler(Token![impl])]
    trait_ty: TypePath,
    #[filler(Token![=>])]
    group: Braced<Punctuated<OperationLine<Token![+]>, Token![;]>>,
}

#[parse_more]
#[allow(unused)]
enum Operation {
    Add(Token![+]),
    Sub(Token![-]),
    Pow(Concat<Token![*], Token![*]>),
    Mul(Token![*]),
    Div(Token![/]),
    Rem(Token![%]),
    Shr(Token![>>]),
    Custom(Ident),
}

#[parse_more]
enum ImplItem {
    Operation(OperationLine<Operation>),
    Group(OperationsGroup),
}

#[proc_macro]
pub fn operation_impl(input: TokenStream) -> TokenStream {
    operations_impl_inner(parse_more_macro_input!(
        input as Punctuated<ImplItem, Token![;]>
    ))
    .into()
}

fn handle_op(
    a: Type,
    op: Option<Operation>,
    b: Type,
    output: Type,
    where_clause: Option<WhereClause>,
    mut custom_trait: Option<TypePath>,
    infer_replace_type: InferReplaceType,
    doc: Option<DocAttributes>,
) -> TokenStream2 {
    let doc = doc.to_doc_attr();
    let mut where_clause = where_clause.unwrap_or(WhereClause {
        where_token: <Token![where]>::default(),
        predicates: Punctuated::new(),
    });

    let mut extractor = GenericsExtractor::new();

    extractor.visit_type(&a);
    extractor.visit_type(&b);
    extractor.visit_type(&output);
    custom_trait
        .as_ref()
        .map(|custom_trait| extractor.visit_type_path(custom_trait));

    let generics = extractor.0;

    let mut compute_where_clauses = ComputeWhereClauses::new(&mut where_clause);
    compute_where_clauses.visit_type(&output);

    if let Err(e) = compute_where_clauses.error {
        return e.into_compile_error();
    }

    if let Some(type_path) = &mut custom_trait {
        let mut replace_visitor = match infer_replace_type {
            InferReplaceType::UnwrapTuple => match &b {
                Type::Tuple(types) => {
                    ReplaceInferedTypes::new(Box::new(types.elems.clone().into_iter()))
                }
                _ => ReplaceInferedTypes::new(Box::new(repeat(b.clone()))),
            },
            _ => ReplaceInferedTypes::new(Box::new(repeat(b.clone()))),
        };
        replace_visitor.visit_type_path_mut(type_path);

        if let Err(e) = replace_visitor.error {
            return e.into_compile_error();
        }
    }

    let generics = generics.iter().fold(TokenStream2::new(), |mut acc, item| {
        acc.extend(quote! {
            #item,
        });
        acc
    });

    let add_impl_generic =
        |op_ident: Option<TokenStream2>, func_ident: Option<TokenStream2>| -> TokenStream2 {
            let func = func_ident.map(|func| {
                quote! {
                    fn #func(self, _rhs: #b) -> Self::Output {
                        unreachable!()
                    }
                }
            });
            let trait_item = custom_trait
                .clone()
                .map_or(quote! {#op_ident<#b>}, ToTokens::into_token_stream);

            quote! {
                #doc
                impl<#generics> #trait_item for #a #where_clause {
                    type Output = #output;

                    #func
                }
            }
        };
    let add_impl = |op_ident, func_ident| add_impl_generic(Some(op_ident), Some(func_ident));

    op.map_or(add_impl_generic(None, None), |op| match op {
        Operation::Add(_) => add_impl(quote! {::core::ops::Add}, quote! {add}),
        Operation::Sub(_) => add_impl(quote! {::core::ops::Sub}, quote! {sub}),
        Operation::Mul(_) => add_impl(quote! {::core::ops::Mul}, quote! {mul}),
        Operation::Div(_) => add_impl(quote! {::core::ops::Div}, quote! {div}),
        Operation::Rem(_) => add_impl(quote! {::core::ops::Rem}, quote! {rem}),
        Operation::Shr(_) => add_impl(quote! {::core::ops::Shr}, quote! {shr}),
        Operation::Pow(_) => add_impl_generic(Some(quote! {crate::type_traits::TypePow}), None),
        Operation::Custom(ident) => match ident.to_string().as_str() {
            "ord" => add_impl_generic(Some(quote! {crate::type_traits::TypeOrd}), None),
            "shl" => add_impl(quote! {::core::ops::Shl}, quote! {shl}),
            _ => syn::Error::new(
                ident.span(),
                format!("\"{}\" is not a valid operation.", ident),
            )
            .into_compile_error(),
        },
    })
}

fn operation_impl_inner(item: ImplItem) -> TokenStream2 {
    match item {
        ImplItem::Operation(concat) => {
            let (doc, args_match, _, out, where_clause) = concat.into();
            args_match
                .0
                .into_iter()
                .map(|arg| {
                    let (a, op, b) = arg.into();
                    handle_op(
                        a.get_type(),
                        Some(op),
                        b,
                        out.clone().get_type(),
                        where_clause.clone(),
                        None,
                        InferReplaceType::UnwrapTuple,
                        doc.clone(),
                    )
                })
                .collect::<TokenStream2>()
        }
        ImplItem::Group(group) => group
            .group
            .value()
            .into_iter()
            .map(|item| {
                let (doc, args_match, _, out, where_clause) = item.into();
                args_match
                    .0
                    .into_iter()
                    .map(|arg| {
                        let (a, _, b) = arg.into();
                        handle_op(
                            a.get_type(),
                            None,
                            b,
                            out.clone().get_type(),
                            where_clause.clone(),
                            Some(group.trait_ty.clone()),
                            InferReplaceType::UnwrapTuple,
                            doc.clone(),
                        )
                    })
                    .collect::<TokenStream2>()
            })
            .collect(),
    }
}
fn operations_impl_inner(operations: Punctuated<ImplItem, Token![;]>) -> TokenStream2 {
    operations
        .into_iter()
        .map(operation_impl_inner)
        .collect::<TokenStream2>()
}

#[parse_more]
#[derive(Clone)]
enum Arguments {
    Arg(TypePath),
    SeveralArgs(Parenthesized<Punctuated<TypePath, Token![,]>>),
}

impl Arguments {
    fn len(&self) -> usize {
        match self {
            Arguments::Arg(_) => 1,
            Arguments::SeveralArgs(punctuated) => punctuated.0.len(),
        }
    }
    fn get_type(self) -> Type {
        match self {
            Arguments::Arg(type_path) => Type::Path(type_path),
            Arguments::SeveralArgs(parenthesized) => {
                let args = parenthesized.value();
                parse_quote!((#args))
            }
        }
    }
}

// TODO implement separated in parse_more
#[derive(Clone)]
struct ArgsMatch<T>(Punctuated<T, syn::token::Or>);
impl<T: ParseMore> ParseMore for ArgsMatch<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args: Punctuated<T, Token![|]> =
            Punctuated::parse_separated_nonempty_with(input, |parse_buffer| {
                Ok(parse_buffer.parse::<ParseMoreWrapper<_>>()?.0)
            })?;
        Ok(Self(args))
    }
}

#[proc_macro]
pub fn application_definition(input: TokenStream) -> TokenStream {
    application_definition_inner(input, true)
}

#[proc_macro]
pub fn application_definition_without_invalid_propagation(input: TokenStream) -> TokenStream {
    application_definition_inner(input, false)
}

fn application_definition_inner(input: TokenStream, propagate_invalid: bool) -> TokenStream {
    let parsed = parse_more_macro_input!(
        input
            as Vec<
                Concat<
                    Option<DocAttributes>,
                    Ident,
                    Option<Parenthesized<Punctuated<Ident, Token![,]>>>,
                    Braced<
                        Punctuated<
                            Concat<
                                Option<DocAttributes>,
                                ArgsMatch<Arguments>,
                                Token![->],
                                Type,
                                Option<WhereClause>,
                            >,
                            Token![;],
                        >,
                    >,
                >,
            >
    );
    parsed
        .into_iter()
        .map(|item| {
            let (doc, name, generics, Braced(definitions)) = item.into();

            let doc = doc.to_doc_attr();

            let generics_names = generics.map_or_else(
                || {
                    let size = definitions.first().map_or(0, |def| {
                        def.clone().into_tuple5().1 .0.first().unwrap().len()
                    });
                    (0..size)
                        .map(|i| {
                            let ident = format_ident!("{}", (b'A' + i as u8) as char);
                            if i == 0 {
                                quote! { #ident }
                            } else {
                                quote! { , #ident }
                            }
                        })
                        .collect()
                },
                |Parenthesized(generics)| quote!(#generics),
            );
            let generics = quote!(<#generics_names>);

            let func_name = format_ident!("{name}Definition");
            let mut result = quote! {
                pub struct #func_name;
                #doc
                pub type #name #generics = Application<#func_name, (#generics_names)>;
            };

            let mut args_count = 0;

            result.extend(
                definitions
                    .into_iter()
                    .map(|definition| {
                        let (doc, args_list, _, output, where_clause) = definition.into();

                        args_list
                            .0
                            .into_iter()
                            .map(|args| {
                                args_count = args_count.max(args.len());
                                handle_op(
                                    parse_quote!(#func_name),
                                    None,
                                    args.get_type(),
                                    output.clone(),
                                    where_clause.clone(),
                                    Some(parse_quote!(FunctionDefinition<_>)),
                                    InferReplaceType::Substitute,
                                    doc.clone(),
                                )
                            })
                            .collect::<TokenStream2>()
                    })
                    .collect::<TokenStream2>(),
            );

            if propagate_invalid {
                if args_count == 1 {
                    result.extend(handle_op(
                        parse_quote!(#func_name),
                        None,
                        parse_quote!(crate::invalid::Invalid),
                        parse_quote! {crate::invalid::Invalid},
                        None,
                        Some(parse_quote!(FunctionDefinition<_>)),
                        InferReplaceType::Substitute,
                        None,
                    ));
                } else {
                    let names = (0..args_count)
                        .map(|i| format_ident!("{}", (b'A' + i as u8) as char))
                        .collect::<Vec<_>>();
                    let mut clauses = WhereClause {
                        where_token: <Token![where]>::default(),
                        predicates: Punctuated::new(),
                    };
                    for i in 0..args_count {
                        if i != 0 {
                            let letter = &names[i - 1];
                            clauses.predicates.push(
                                parse_quote! {#letter: crate::bool::IsNot<crate::invalid::Invalis>},
                            );
                        }
                        let ty = names.iter().enumerate().fold(quote! {}, |acc, (j, val)| {
                            if i == j {
                                quote! {#acc crate::invalid::Invalid, }
                            } else {
                                quote! {#acc #val, }
                            }
                        });
                        let ty = parse_quote! {(#ty)};
                        result.extend(handle_op(
                            parse_quote!(#func_name),
                            None,
                            ty,
                            parse_quote! {crate::invalid::Invalid},
                            Some(clauses.clone()),
                            Some(parse_quote!(FunctionDefinition<_>)),
                            InferReplaceType::Substitute,
                            None,
                        ));
                    }
                }
            }

            result
        })
        .collect::<TokenStream2>()
        .into()
}
