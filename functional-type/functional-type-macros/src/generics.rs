use std::collections::HashSet;

use syn::{
    spanned::Spanned, visit::Visit, visit_mut::VisitMut, Error, Ident, Result, Type, TypePath,
};

const STRUCT_ITEMS: &[&str] = &[
    "Invalid",
    "Zero",
    "One",
    "U1",
    "U2",
    "UIntDelimiter",
    "True",
    "False",
    "Greater",
    "Equal",
    "Less",
];

pub struct GenericsExtractor(pub HashSet<Ident>);

impl GenericsExtractor {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl<'a> Visit<'a> for GenericsExtractor {
    fn visit_type_path(&mut self, path: &'a TypePath) {
        if let Some(ident) = path.path.get_ident() {
            let ident_name = ident.to_string();
            if !STRUCT_ITEMS.contains(&ident_name.as_str()) && !ident_name.ends_with("Definition") {
                self.0.insert(ident.clone());
            }
        }
        syn::visit::visit_type_path(self, path);
    }
}

pub struct ReplaceInferedTypes {
    replace: Box<dyn Iterator<Item = Type>>,
    pub error: Result<()>,
}

impl ReplaceInferedTypes {
    pub fn new(replace: Box<dyn Iterator<Item = Type>>) -> Self {
        Self {
            replace,
            error: Ok(()),
        }
    }
}

impl VisitMut for ReplaceInferedTypes {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if self.error.is_err() {
            return;
        }
        if let Type::Infer(_) = ty {
            match self.replace.next() {
                Some(replace) => {
                    *ty = replace;
                }
                None => {
                    self.error = Err(Error::new(
                        ty.span(),
                        "There is not enough types to fill infered places !",
                    ))
                }
            }
        }
        syn::visit_mut::visit_type_mut(self, ty);
    }
}

pub enum InferReplaceType {
    Substitute,
    UnwrapTuple,
}
