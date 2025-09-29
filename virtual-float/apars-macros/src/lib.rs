#![feature(macro_metavar_expr, macro_metavar_expr_concat)]

use apars_proc_macros::make_variant;

#[macro_export]
macro_rules! op_variant {
    ($op:ident, $op_trait:ident, $base:ty, $rhs:ty) => {
        impl std::ops::$op_trait<&$rhs> for $base {
            type Output = $base;

            fn $op(self, rhs: &$rhs) -> Self::Output {
                <$base as std::ops::$op_trait<$rhs>>::$op(self, rhs.clone())
            }
        }
        impl std::ops::$op_trait<$rhs> for &$base {
            type Output = $base;

            fn $op(self, rhs: $rhs) -> Self::Output {
                <$base as std::ops::$op_trait<$rhs>>::$op(self.clone(), rhs)
            }
        }
        impl std::ops::$op_trait<&$rhs> for &$base {
            type Output = $base;

            fn $op(self, rhs: &$rhs) -> Self::Output {
                <$base as std::ops::$op_trait<$rhs>>::$op(self.clone(), rhs.clone())
            }
        }

        impl std::ops::${concat($op_trait, Assign)}<$rhs> for $base {
            fn ${concat($op, _assign)}(&mut self, rhs: $rhs) {
                *self = <&$base as std::ops::$op_trait<$rhs>>::$op(&*self, rhs)
            }
        }
        impl std::ops::${concat($op_trait, Assign)}<&$rhs> for $base {
            fn ${concat($op, _assign)}(&mut self, rhs: &$rhs) {
                *self = <&$base as std::ops::$op_trait<&$rhs>>::$op(&*self, rhs)
            }
        }
    };
}

make_variant! {
    (Add, {
        self + Self::from(rhs)
    }, {
        self + Self::from(rhs)
    }),
    (Sub, {
        self - Self::from(rhs)
    }, {
        self + Self::from(rhs)
    }),
    (Mul, {
        self * Self::from(rhs)
    }, {
        self + Self::from(rhs)
    }),
    (Div, {
        self / Self::from(rhs)
    }, {
        self + Self::from(rhs)
    }),
    (Shr, {
        self >> rhs as usize
    }, {
        if rhs < 0 {
            self << rhs.unsigned_abs()
        } else {
            self >> rhs.unsigned_abs()
        }
    }),
    (Shl, {
        self << rhs as usize
    }, {
        if rhs < 0 {
            self >> rhs.unsigned_abs()
        } else {
            self << rhs.unsigned_abs()
        }
     })
}
