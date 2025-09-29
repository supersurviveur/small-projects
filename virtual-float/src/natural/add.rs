use std::ops::Add;

use apars_macros::{add_unsigned, add_variant};

use crate::natural::Natural;

impl Add for Natural {
    type Output = Natural;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        if self.chunk_count() < rhs.chunk_count() {
            (self, rhs) = (rhs, self);
        }
        let mut carry = false;
        for i in 0..self.chunk_count() {
            let tmp;
            (self.inner[i], tmp) = self.inner[i].overflowing_add(carry as u64);
            if i < rhs.chunk_count() {
                (self.inner[i], carry) = self.inner[i].overflowing_add(rhs.inner[i]);
            } else {
                carry = false;
            }
            carry |= tmp;
        }
        if carry {
            self.inner.push(1);
        }
        self
    }
}

add_variant! { Natural, Natural }
add_unsigned! { Natural }

macro_rules! add_signed_natural {
    ($($from:ty),*) => {
        $(
            impl std::ops::Add<$from> for Natural {
                type Output = Natural;

                fn add(self, rhs: $from) -> Self::Output {
                    if rhs < 0 {
                        self - Natural::try_from(rhs.abs()).unwrap()
                    } else {
                        self + Natural::try_from(rhs).unwrap()
                    }
                }
            }
        )*
    };
}

add_signed_natural! {
    i8, i16, i32, i64, isize
}
