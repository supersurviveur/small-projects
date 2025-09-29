use apars_macros::{sub_unsigned, sub_variant};

use crate::natural::Natural;

impl std::ops::Sub for Natural {
    type Output = Natural;

    fn sub(mut self, rhs: Self) -> Self::Output {
        if self.chunk_count() < rhs.chunk_count() {
            panic!("Can't substract a Natural from a smaller Natural")
        }
        let mut carry = false;
        for i in 0..self.chunk_count() {
            let tmp;
            (self.inner[i], tmp) = self.inner[i].overflowing_sub(carry as u64);
            if i < rhs.chunk_count() {
                (self.inner[i], carry) = self.inner[i].overflowing_sub(rhs.inner[i]);
            } else {
                carry = false;
            }
            carry |= tmp;
        }
        if carry {
            panic!("Can't substract a Natural from a smaller Natural")
        }
        self.fit();
        self
    }
}

sub_variant! { Natural, Natural }
sub_unsigned! { Natural }

macro_rules! sub_signed_natural {
    ($($from:ty),*) => {
        $(
            impl std::ops::Sub<$from> for Natural {
                type Output = Natural;

                fn sub(self, rhs: $from) -> Self::Output {
                    if rhs < 0 {
                        self + Natural::try_from(rhs.abs()).unwrap()
                    } else {
                        self - Natural::try_from(rhs).unwrap()
                    }
                }
            }
        )*
    };
}

sub_signed_natural! {
    i8, i16, i32, i64, isize
}
