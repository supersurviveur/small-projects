use apars_macros::{mul_unsigned, mul_variant};

use crate::natural::Natural;

impl std::ops::Mul for Natural {
    type Output = Natural;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Natural::zero();
        if rhs.is_zero() || self.is_zero() {
            return result;
        }

        let mut multiplier = self.clone();
        let mut prev_shift = 0;
        for i in 0..rhs.ilog2() + 1 {
            if rhs.inner[i / 64] & 1 << (i % 64) != 0 {
                multiplier <<= i - prev_shift;
                result += multiplier.clone();
                prev_shift = i;
            }
        }
        result
    }
}

mul_variant! { Natural, Natural }
mul_unsigned! { Natural }
