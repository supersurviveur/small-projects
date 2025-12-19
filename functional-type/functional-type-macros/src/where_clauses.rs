use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_quote, visit::Visit, GenericArgument, Ident, Result, WhereClause};

pub struct ComputeWhereClauses<'a> {
    pub clauses: &'a mut WhereClause,
    pub current_ident: Ident,
    pub error: Result<()>,
    current_args: Option<&'a syn::AngleBracketedGenericArguments>,
}

impl<'a> ComputeWhereClauses<'a> {
    pub fn new(clauses: &'a mut WhereClause) -> Self {
        Self {
            clauses,
            current_ident: format_ident!("Unreachable"),
            error: Ok(()),
            current_args: None,
        }
    }
    fn where_clause_rule<'b, const N: usize>(&'b self) -> Result<[&'a GenericArgument; N]> {
        Ok(self
            .current_args
            .unwrap()
            .args
            .iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| {
                syn::Error::new(
                    self.current_ident.span(),
                    format!("{} must have {N} generic arguments", self.current_ident),
                )
            })?)
    }
    fn add_constraint(&mut self, ty: TokenStream2, constraints: TokenStream2) {
        self.clauses
            .predicates
            .push(parse_quote!(#ty: #constraints));
    }
    fn visit_angle_bracketed_generic_arguments_inner(
        &mut self,
        args: &'a syn::AngleBracketedGenericArguments,
    ) -> Result<()> {
        self.current_args = Some(args);
        match self.current_ident.to_string().as_str() {
            "If" => {
                let [cond, then, else_body] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::IfDefinition},
                    quote! {crate::application::FunctionDefinition<(#cond, #then, #else_body)>},
                );
            }
            "Then" => {
                let [first, then] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::ThenDefinition},
                    quote! {crate::application::FunctionDefinition<(#first, #then)>},
                );
            }
            "AreEqual" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::IsOrdEqualDefinition},
                    quote! {crate::application::FunctionDefinition<Compare<#a, #b>>},
                );
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeOrd<#b>});
            }
            "IsGreater" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::IsOrdGreaterDefinition},
                    quote! {crate::application::FunctionDefinition<Compare<#a, #b>>},
                );
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeOrd<#b>});
            }
            "IsGreaterOrEqual" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::IsOrdGreaterOrEqualDefinition},
                    quote! {crate::application::FunctionDefinition<Compare<#a, #b>>},
                );
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeOrd<#b>});
            }
            "IsLess" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(
                    quote! {crate::bool::IsOrdLessDefinition},
                    quote! {crate::application::FunctionDefinition<Compare<#a, #b>>},
                );
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeOrd<#b>});
            }
            "IsEven" => {
                let [a] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeEven});
            }
            "Sum" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Add<#b>});
            }
            "Diff" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Sub<#b>});
            }
            "Product" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Mul<#b>});
            }
            "Divide" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Div<#b>});
            }
            "Remainder" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Rem<#b>});
            }
            "ShiftRight" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Shr<#b>});
            }
            "ShiftLeft" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {core::ops::Shl<#b>});
            }
            "Pow" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypePow<#b>});
            }
            "ILog2" => {
                let [a] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeILog2});
            }
            "Trim" => {
                let [value] = self.where_clause_rule()?;
                self.add_constraint(quote! {#value}, quote! {crate::type_traits::Trim});
            }
            "Compare" => {
                let [a, b] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::TypeOrd<#b>});
            }
            "PrivateDivide" => {
                let [a, d, s] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::PrivateDiv<#d, #s>});
            }
            "PrivateRemainder" => {
                let [a, d, s] = self.where_clause_rule()?;
                self.add_constraint(quote! {#a}, quote! {crate::type_traits::PrivateRem<#d, #s>});
            }
            _ => {
                syn::visit::visit_angle_bracketed_generic_arguments(self, args);
            }
        }
        args.args
            .iter()
            .for_each(|arg| self.visit_generic_argument(arg));
        Ok(())
    }
}

impl<'a> Visit<'a> for ComputeWhereClauses<'a> {
    fn visit_angle_bracketed_generic_arguments(
        &mut self,
        args: &'a syn::AngleBracketedGenericArguments,
    ) {
        if self.error.is_err() {
            return;
        }
        if let Err(e) = self.visit_angle_bracketed_generic_arguments_inner(args) {
            self.error = Err(e);
        }
    }
    fn visit_path_segment(&mut self, segment: &'a syn::PathSegment) {
        self.current_ident = segment.ident.clone();
        syn::visit::visit_path_segment(self, segment);
    }
}
