use core::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

use crate::type_traits::{PrivateDiv, PrivateRem, TypeILog2, TypeOrd, TypePow};

pub type Sum<A, B> = <A as Add<B>>::Output;
pub type Diff<A, B> = <A as Sub<B>>::Output;
pub type Product<A, B> = <A as Mul<B>>::Output;
pub type Divide<A, B> = <A as Div<B>>::Output;
pub type Remainder<A, B> = <A as Rem<B>>::Output;
pub type Pow<A, B> = <A as TypePow<B>>::Output;
pub type ShiftRight<A, B> = <A as Shr<B>>::Output;
pub type ShiftLeft<A, B> = <A as Shl<B>>::Output;

pub type ILog2<T> = <T as TypeILog2>::Output;

pub type Compare<A, B> = <A as TypeOrd<B>>::Output;

pub type Trim<T> = <T as crate::type_traits::Trim>::Output;
pub type IsEven<T> = <T as crate::type_traits::TypeEven>::Output;

pub(crate) type PrivateDivide<A, B, Shift> = <A as PrivateDiv<B, Shift>>::Output;
pub(crate) type PrivateRemainder<A, B, Shift> = <A as PrivateRem<B, Shift>>::Output;
