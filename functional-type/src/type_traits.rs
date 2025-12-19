use core::{
    convert::Infallible,
    fmt::{Binary, Debug, Display},
    marker::PhantomData,
};

pub trait TypeInto<T> {
    fn type_into() -> T;
}

pub trait TypeFrom<T> {
    fn type_from() -> Self;
}

impl<U, T: TypeFrom<U>> TypeInto<T> for U {
    fn type_into() -> T {
        T::type_from()
    }
}

#[derive(Debug)]
pub struct TryFromTypeError;

pub trait TypeTryInto<T> {
    type Error;
    fn type_try_into() -> Result<T, Self::Error>;
}

pub trait TypeTryFrom<T>: Sized {
    type Error;
    fn type_try_from() -> Result<Self, Self::Error>;
}

impl<U, T: TypeTryFrom<U>> TypeTryInto<T> for U {
    type Error = T::Error;

    fn type_try_into() -> Result<T, Self::Error> {
        T::type_try_from()
    }
}

impl<U, T: TypeFrom<U>> TypeTryFrom<U> for T {
    type Error = Infallible;

    fn type_try_from() -> Result<Self, Self::Error> {
        Ok(T::type_from())
    }
}

pub struct TypeDisplayWrapper<T: ?Sized>(PhantomData<T>);
pub trait TypeDebug {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}
pub trait TypeDisplay {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}
pub trait TypeBinary {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

pub trait ToTypeDisplayWrapper {
    fn display() -> TypeDisplayWrapper<Self>;
}

impl<T> ToTypeDisplayWrapper for T {
    fn display() -> TypeDisplayWrapper<Self> {
        TypeDisplayWrapper(PhantomData)
    }
}

impl<T: TypeDebug> Debug for TypeDisplayWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as TypeDebug>::fmt(f)
    }
}
impl<T: TypeDisplay> Display for TypeDisplayWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as TypeDisplay>::fmt(f)
    }
}
impl<T: TypeBinary> Binary for TypeDisplayWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as TypeBinary>::fmt(f)
    }
}

pub trait TypeOrd<B> {
    type Output;
}

pub trait Trim {
    type Output;
}

pub trait TypePow<B> {
    type Output;
}

pub trait TypeEven {
    type Output;
}

pub trait TypeILog2 {
    type Output;
}

pub trait PrivateDiv<D, ShiftLevel> {
    type Output;
}
pub trait PrivateRem<D, ShiftLevel> {
    type Output;
}
