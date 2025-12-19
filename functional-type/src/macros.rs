use crate::type_traits::{ToTypeDisplayWrapper, TypeDebug};

#[macro_export]
macro_rules! assert_ty_eq {
    ($a:ty, $b:ty $(,)?) => {
        if <$crate::bool::AreNotEqual<$a, $b> as $crate::type_traits::TypeInto<bool>>::type_into() {
            $crate::macros::ty_assert_failed::<$a, $b>(
                $crate::macros::TyAssertKind::Eq,
                core::option::Option::None,
            )
        }
    };
    ($a:ty, $b:ty, $($arg:tt)+) => {
        if <$crate::bool::AreNotEqual<$a, $b> as $crate::type_traits::TypeInto<bool>>::type_into() {
            $crate::macros::ty_assert_failed::<$a, $b>(
                $crate::macros::TyAssertKind::Eq,
                core::option::Option::Some(core::format_args!($($arg)+)),
            )
        }
    };
}

#[macro_export]
macro_rules! assert_ty_ne {
    ($a:ty, $b:ty $(,)?) => {
        if <$crate::bool::AreEqual<$a, $b> as $crate::type_traits::TypeInto<bool>>::type_into() {
            $crate::macros::ty_assert_failed::<$a, $b>(
                $crate::macros::TyAssertKind::Ne,
                core::option::Option::None,
            )
        }
    };
    ($a:ty, $b:ty, $($arg:tt)+) => {
        if <$crate::bool::AreEqual<$a, $b> as $crate::type_traits::TypeInto<bool>>::type_into() {
            $crate::macros::ty_assert_failed::<$a, $b>(
                $crate::macros::TyAssertKind::Ne,
                core::option::Option::Some(core::format_args!($($arg)+)),
            )
        }
    };
}

pub enum TyAssertKind {
    Eq,
    Ne,
    Match,
}

pub fn ty_assert_failed<Left: TypeDebug, Right: TypeDebug>(
    kind: TyAssertKind,
    args: Option<core::fmt::Arguments<'_>>,
) -> ! {
    let op = match kind {
        TyAssertKind::Eq => "==",
        TyAssertKind::Ne => "!=",
        TyAssertKind::Match => "matches",
    };

    match args {
        Some(args) => panic!(
            r#"type assertion `left {op} right` failed: {args}
  left: {:?}
 right: {:?}"#,
            <Left as ToTypeDisplayWrapper>::display(),
            <Right as ToTypeDisplayWrapper>::display()
        ),
        None => panic!(
            r#"type assertion `left {op} right` failed
  left: {:?}
 right: {:?}"#,
            <Left as ToTypeDisplayWrapper>::display(),
            <Right as ToTypeDisplayWrapper>::display()
        ),
    }
}
