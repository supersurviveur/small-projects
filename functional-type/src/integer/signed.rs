use crate::bool::{If, IsGreater, IsGreaterOrEqual};
use crate::type_traits::{TypeDisplay, TypeInto};
use crate::{
    bool::{False, True},
    operators::{Compare, Diff, Sum},
};
use functional_type_macros::{generate_signed_integers, operation_impl};

pub struct SInt<S, U>(S, U);

generate_signed_integers! { 1024 }

impl<S: TypeInto<bool>, U: TypeDisplay> TypeDisplay for SInt<S, U> {
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if S::type_into() {
            write!(f, "-")?;
        }
        <U as TypeDisplay>::fmt(f)
    }
}

operation_impl! {
    // ----------------------
    // ADDITION
    // ----------------------

    SInt<S, UL> + SInt<S, UR> = SInt<S, Sum<UL, UR>>;

    SInt<False, UL> + SInt<True, UR> = If<
        IsGreaterOrEqual<UL, UR>,
        SInt<False, Diff<UL, UR>>,
        SInt<True, Diff<UR, UL>>
    >;

    SInt<True, UL> + SInt<False, UR> = If<
        IsGreater<UL, UR>,
        SInt<True, Diff<UL, UR>>,
        SInt<False, Diff<UR, UL>>,
    >;
}
