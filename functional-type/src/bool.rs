use functional_type_macros::{
    application_definition, application_definition_without_invalid_propagation, operation_impl,
};

use crate::{
    application::{Application, FunctionDefinition},
    invalid::Invalid,
    operators::Compare,
    type_traits::{TypeBinary, TypeDebug, TypeDisplay, TypeFrom, TypeOrd},
};

pub struct False;
pub struct True;

pub trait Bool {}
impl Bool for False {}
impl Bool for True {}

impl TypeFrom<True> for bool {
    fn type_from() -> Self {
        true
    }
}
impl TypeFrom<False> for bool {
    fn type_from() -> Self {
        false
    }
}

impl TypeBinary for False {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0")
    }
}
impl TypeBinary for True {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "1")
    }
}

impl TypeDisplay for False {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "false")
    }
}
impl TypeDisplay for True {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "true")
    }
}
impl TypeDebug for False {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "False")
    }
}
impl TypeDebug for True {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "True")
    }
}

operation_impl! {
    // ----------------------
    // COMPARISON
    // ----------------------

    False ord True = Less;

    True ord False = Greater;
}

pub trait Is<T> {}
impl<T> Is<T> for T {}
pub trait IsNot<T> {}
impl<A, B> IsNot<B> for A
where
    Compare<A, B>: IsNotEqual,
    A: TypeOrd<B>,
{
}

pub struct Less;
pub struct Equal;
pub struct Greater;

trait IsNotEqual {}

impl IsNotEqual for Less {}
impl IsNotEqual for Greater {}
impl IsNotEqual for Invalid {}

impl TypeDebug for Less {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Less")
    }
}
impl TypeDebug for Equal {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Equal")
    }
}
impl TypeDebug for Greater {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Greater")
    }
}

application_definition! {
    Not {
        False -> True;
        True -> False;
    }
    IsOrdLess {
        Less -> True;
        Equal | Greater -> False;
    }
    IsOrdGreater {
        Greater -> True;
        Equal | Less -> False;
    }
    IsOrdEqual {
        Equal -> True;
        Greater | Less -> False;
    }
    IsOrdLessOrEqual {
        Equal | Less -> True;
        Greater -> False;
    }
    IsOrdGreaterOrEqual {
        Equal | Greater -> True;
        Less -> False;
    }

}
application_definition_without_invalid_propagation! {
    /// e
    /// e
    Then {
        (Less, T) -> Less;
        (Greater, T) -> Greater;
        (Equal, T) -> T;
        (Invalid, T) -> Invalid;
    }
}

pub type AreEqual<A, B> = IsOrdEqual<Compare<A, B>>;
pub type AreNotEqual<A, B> = Not<AreEqual<A, B>>;
pub type IsGreater<A, B> = IsOrdGreater<Compare<A, B>>;
pub type IsLess<A, B> = IsOrdLess<Compare<A, B>>;
pub type IsGreaterOrEqual<A, B> = IsOrdGreaterOrEqual<Compare<A, B>>;
pub type IsLessOrEqual<A, B> = IsOrdLessOrEqual<Compare<A, B>>;

application_definition_without_invalid_propagation! {
    If(Cond, Then, Else) {
        (True, Then, Else) -> Then;
        (False, Then, Else) -> Else;
        (Invalid, Then, Else) -> Invalid;
    }
}
