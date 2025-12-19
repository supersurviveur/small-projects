use crate::{bool::True, type_traits::PrivateDiv};
use crate::integer::unsigned::UIntDelimiter;
use functional_type_macros::operation_impl;

use crate::{
    bool::{Equal, False, Greater, Less},
    integer::unsigned::UInt,
    type_traits::{TypeDebug, TypeDisplay},
};

pub struct Invalid;

impl TypeDisplay for Invalid {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid")
    }
}
impl TypeDebug for Invalid {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid")
    }
}

macro_rules! ord_invalid_propagation {
    ($($elem:ty),*) => {
        operation_impl! {
            $(
                Invalid ord $elem = Invalid;
                $elem ord Invalid = Invalid;
            )*
        }
    };
}

ord_invalid_propagation! {
    True,
    False,
    UIntDelimiter,
    Equal,
    Less,
    Greater,
    UInt<U, B>
}

operation_impl! {
    Invalid + T = Invalid;
    Invalid - T = Invalid;
    Invalid * T = Invalid;
    Invalid / T = Invalid;
    Invalid % T = Invalid;
    Invalid >> T = Invalid;
    Invalid shl T = Invalid;
    Invalid ord Invalid = Equal;

    impl PrivateDiv<_, _> => {
        Invalid + (A, B) = Invalid;
    }
}
