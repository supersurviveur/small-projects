use core::fmt::Debug;
use core::ops::{Div, Rem, Shl};

use crate::bool::{Bool, Is, IsNot, Then};
use crate::operators::{Divide, ILog2, IsEven, Pow, PrivateDivide, PrivateRemainder, Remainder, ShiftLeft};
use crate::type_traits::{PrivateDiv, PrivateRem, TypeILog2, TypeInto};
use crate::{
    bool::{Equal, False, Greater, If, IsGreaterOrEqual, Less, True},
    invalid::Invalid,
    operators::{Compare, Diff, Product, ShiftRight, Sum, Trim},
    type_traits::{TryFromTypeError, TypeBinary, TypeDebug, TypeDisplay, TypeTryInto},
};

use functional_type_macros::{generate_unsigned_integers, operation_impl};

pub struct UInt<U, B>(U, B);
pub struct UIntDelimiter;

pub trait NotADelimiter {}
impl<U, B> NotADelimiter for UInt<U, B> {}

generate_unsigned_integers! { 2048 }

pub type Zero = U0;
pub type One = U1;

macro_rules! integer_try_into_impl {
    ($($ty:ty),*) => {
        $(impl TypeTryInto<$ty> for Zero {
            type Error = TryFromTypeError;

            fn type_try_into() -> Result<$ty, Self::Error> {
                Ok(0)
            }
        }

        impl<U: TypeTryInto<$ty, Error = TryFromTypeError>, B: TypeInto<bool>> TypeTryInto<$ty>
            for UInt<U, B>
        {
            type Error = TryFromTypeError;
            fn type_try_into() -> Result<$ty, Self::Error> {
                U::type_try_into().and_then(|u| {
                    u.checked_mul(2)
                        .ok_or(TryFromTypeError)?
                        .checked_add(B::type_into() as $ty)
                        .ok_or(TryFromTypeError)
                })
            }
        })*
    };
}
integer_try_into_impl! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, i128 }

impl TypeDisplay for UIntDelimiter {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0")
    }
}
impl TypeDebug for UIntDelimiter {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UIntDelimiter")
    }
}

trait NumberDisplayBase {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}
macro_rules! display_base_case {
    ($(($t:ty, $val:literal)),*) => {
        $(impl NumberDisplayBase for $t {
            fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", $val)
            }
        })*
    };
}
display_base_case! {
    (U0, 0),
    (U1, 1),
    (U2, 2),
    (U3, 3),
    (U4, 4),
    (U5, 5),
    (U6, 6),
    (U7, 7),
    (U8, 8),
    (U9, 9)
}

// impl<U, B> TypeDisplay for UInt<U, B>
// where
//     Remainder<Self, U10>: NumberDisplayBase,
//     Divide<Self, U10>: TypeDisplay,
//     Self: Div<U10>,
//     Self: Rem<U10>,
//     UInt<U, B>: core::ops::Sub<One>,
// {
//     fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         <Divide<Self, U10> as TypeDisplay>::fmt(f)?;
//         <Remainder<Self, U10> as NumberDisplayBase>::fmt(f)
//     }
// }

impl<UR, BR, BL: Bool> AsRef<BL> for UInt<UR, BR>
where
    UInt<UR, True>: Div<U10>,
    // UInt<UR, True>: PrivateDiv<ShiftLeft<U1, ILog2<UInt<UR, True>>>, ILog2<UInt<UR, True>>>,
    // UR: TypeILog2,
    // <UR as TypeILog2>::Output: core::ops::Add<One>,
    // One: Shl<Sum<ILog2<UR>, One>>,
    // U1: Shl<UInt<UL, BL>>,
    // U2: Shl<Diff<UInt<UL, BL>, One>>,
    // UInt<UIntDelimiter, BL>: core::ops::Sub<One>,
{
    fn as_ref(&self) -> &BL {
        todo!()
    }
}
impl<U: TypeTryInto<u64, Error = TryFromTypeError>, B: TypeInto<bool>> TypeDisplay for UInt<U, B> {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            <Self as TypeTryInto<u64>>::type_try_into().unwrap()
        )
    }
}

impl<B: TypeBinary> TypeBinary for UInt<UIntDelimiter, B> {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <B as TypeBinary>::fmt(f)
    }
}

impl<U: TypeBinary + NotADelimiter, B: TypeBinary> TypeBinary for UInt<U, B> {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <U as TypeBinary>::fmt(f)?;
        <B as TypeBinary>::fmt(f)
    }
}
impl TypeBinary for UIntDelimiter {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0")
    }
}

impl<U, B> TypeDebug for UInt<U, B>
where
    U: TypeDebug,
    B: TypeDebug,
{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UInt<")?;
        <U as TypeDebug>::fmt(f)?;
        write!(f, ", ")?;
        <B as TypeDebug>::fmt(f)?;
        write!(f, ">")
    }
}

operation_impl! {
    // ----------------------
    // TRIM
    // ----------------------
    impl crate::type_traits::Trim => {
        UIntDelimiter + _ = UIntDelimiter;

        UInt<UIntDelimiter, False> + _ = UIntDelimiter;

        UInt<UIntDelimiter, True> + _ = UInt<UIntDelimiter, True>;

        UInt<UInt<U, B>, B2> + _ = UInt<UInt<U, B>, B2>;

        UInt<Invalid, B> + _ = Invalid;
    };

    // ----------------------
    // COMPARISON
    // ----------------------
    UIntDelimiter ord UIntDelimiter = Equal;

    UIntDelimiter ord UInt<UR, B> = Less;

    UInt<U, B> ord UIntDelimiter = Greater;

    UInt<UL, B> ord UInt<UR, B> = Compare<UL, UR>;

    UInt<UL, True> ord UInt<UR, False> = Then<Compare<UL, UR>, Greater>;

    UInt<UL, False> ord UInt<UR, True> = Then<Compare<UL, UR>, Less>;

    // ----------------------
    // EVEN
    // ----------------------
    impl crate::type_traits::TypeEven => {
        Zero + _ = True;

        UInt<U, False> + _ = True;

        UInt<U, True> + _ = False;
    };

    // ----------------------
    // ADDITION
    // ----------------------
    Zero + Zero = Zero;

    Zero + UInt<U, B> = UInt<U, B>;

    UIntDelimiter + B = UInt<UIntDelimiter, B> where
        B: Bool;

    UInt<U, B> + False = UInt<U, B>;

    UInt<U, False> + True = UInt<U, True>;

    UInt<U, True> + True = UInt<Sum<U, True>, False>;

    UInt<U, B> + UIntDelimiter = UInt<U, B>;

    UInt<UL, False> + UInt<UR, False> = UInt<Sum<UL, UR>, False>;

    UInt<UL, True> + UInt<UR, False> = UInt<Sum<UL, UR>, True>;

    UInt<UL, False> + UInt<UR, True> = UInt<Sum<UL, UR>, True>;

    UInt<UL, True> + UInt<UR, True> = UInt<Sum<Sum<UL, UR>, True>, False>;

    // ----------------------
    // SUBTRACTION
    // ----------------------

    UIntDelimiter - UInt<U, B> = Invalid;
    UIntDelimiter - True = Invalid;

    Zero - Zero = Zero;

    UInt<U, B> - False = UInt<U, B>;

    UInt<U, True> - True = Trim<UInt<U, False>>;

    UInt<U, False> - True = Trim<UInt<Diff<U, True>, True>>;

    UInt<U, B> - UIntDelimiter = UInt<U, B>;

    UInt<UL, False> - UInt<UR, False> = Trim<UInt<Diff<UL, UR>, False>>;

    UInt<UL, True> - UInt<UR, False> = Trim<UInt<Diff<UL, UR>, True>>;

    UInt<UL, True> - UInt<UR, True> = Trim<UInt<Diff<UL, UR>, False>>;

    UInt<UL, False> - UInt<UR, True> = Trim<UInt<Diff<Diff<UL, UR>, True>, True>>;

    // ----------------------
    // MULTIPLICATION
    // ----------------------

    Zero * Zero = Zero;

    UInt<U, B> * False = False;

    UInt<U, B> * True = UInt<U, B>;

    UInt<U, B> * UIntDelimiter = Zero;

    UInt<UL, B> * UInt<UR, False> = Product<UInt<UInt<UL, B>, False>, UR>;

    UInt<UL, B> * UInt<UR, True> = Sum<Product<UInt<UInt<UL, B>, False>, UR>, UInt<UL, B>>;

    // ----------------------
    // SHIFT RIGHT
    // ----------------------

    Zero >> Zero = Zero;

    UInt<UL, BL> >> Zero = UInt<UL, BL>;

    Zero >> UInt<UR, BR> = Zero;

    UInt<UL, BL> >> UInt<UR, BR> = ShiftRight<UL, Diff<UInt<UR, BR>, One>>;

    // ----------------------
    // SHIFT LEFT
    // ----------------------

    Zero shl Zero = Zero;

    UInt<UL, BL> shl Zero = UInt<UL, BL>;

    Zero shl UInt<UR, BR> = Zero;

    UInt<UL, BL> shl UInt<UR, BR> = ShiftLeft<UInt<UInt<UL, BL>, False>, Diff<UInt<UR, BR>, One>>;

    // ----------------------
    // ILOG2
    // ----------------------

    impl crate::type_traits::TypeILog2 => {
        Zero + _ = Zero;

        UInt<U, B> + _ = Sum<ILog2<U>, One>;
    };

    // ----------------------
    // DIVISION
    // ----------------------

    Zero / UInt<U, B> = Zero;

    UInt<UL, BL> / UInt<UR, BR> = If<
        IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
        PrivateDivide<
            UInt<UL, BL>,
            ShiftLeft<UInt<UR, BR>, Diff<ILog2<UInt<UL, BL>>, ILog2<UInt<UR, BR>>>>,
            Diff<ILog2<UInt<UL, BL>>, ILog2<UInt<UR, BR>>>
        >,
        Zero
    >;

    impl PrivateDiv<_, _> => {
        UIntDelimiter + (UInt<U, B>, Zero) | UIntDelimiter + (UInt<U, B>, UInt<US, BS>) = Zero;

        UInt<UL, BL> + (UInt<UR, BR>, Zero) = If<
            IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
            One,
            Zero
        >;

        UInt<UL, BL> + (UInt<UR, BR>, UInt<US, BS>) = Sum<
            PrivateDivide<
                If<
                    IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
                    Diff<UInt<UL, BL>, UInt<UR, BR>>,
                    UInt<UL, BL>
                >,
                ShiftRight<UInt<UR, BR>, One>,
                Diff<UInt<US, BS>, One>
            >,
            If<
                IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
                ShiftLeft<One, UInt<US, BS>>,
                Zero
            >
        >;
    };

    // ----------------------
    // REMAINDER
    // ----------------------

    UIntDelimiter % UInt<U, B> = Zero;

    UInt<UL, BL> % UInt<UR, BR> = If<
        IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
        PrivateRemainder<
            UInt<UL, BL>,
            ShiftLeft<UInt<UR, BR>, Diff<ILog2<UInt<UL, BL>>, ILog2<UInt<UR, BR>>>>,
            Diff<ILog2<UInt<UL, BL>>, ILog2<UInt<UR, BR>>>
        >,
        UInt<UL, BL>
    >;

    impl PrivateRem<_, _> => {
        UInt<U, B> + (Invalid, Invalid) = Invalid;
        
        UIntDelimiter + (UInt<U, B>, Zero) | UIntDelimiter + (UInt<U, B>, UInt<US, BS>) = Zero;

        UInt<UL, BL> + (UInt<UR, BR>, Zero) = If<
            IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
            Diff<UInt<UL, BL>, UInt<UR, BR>>,
            UInt<UL, BL>
        >;

        UInt<UL, BL> + (UInt<UR, BR>, UInt<US, BS>) =
            PrivateRemainder<
                If<
                    IsGreaterOrEqual<UInt<UL, BL>, UInt<UR, BR>>,
                    Diff<UInt<UL, BL>, UInt<UR, BR>>,
                    UInt<UL, BL>
                >,
                ShiftRight<UInt<UR, BR>, One>,
                Diff<UInt<US, BS>, One>
            >;
    };

    // ----------------------
    // POW
    // ----------------------

    UInt<UL, BL> ** Zero = One;

    UInt<UL, BL> ** UInt<UR, BR> = If<
        IsEven<UInt<UR, BR>>,
        Pow<Product<UInt<UL, BL>, UInt<UL, BL>>, ShiftRight<UInt<UR, BR>, One>>,
        Product<UInt<UL, BL>, Pow<Product<UInt<UL, BL>, UInt<UL, BL>>, ShiftRight<UInt<UR, BR>, One>>>
    >;

    // ----------------------
    // INVALID
    // ----------------------

    UInt<U, B> shl Invalid = Invalid;
    impl PrivateDiv<_, _> => {
        UInt<U, B> + (Invalid, Invalid) = Invalid;
    }
}
