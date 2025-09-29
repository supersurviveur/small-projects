use apars_macros::{div_unsigned, div_variant};

use crate::natural::Natural;

impl Natural {
    pub fn quot_rem(mut self, rhs: Self) -> (Natural, Natural) {
        let mut quotient = Natural::zero();
        if self < rhs {
            return (quotient, self);
        }

        let self_highest_bit = self.ilog2() + 1;
        let mut current_shift = self_highest_bit - (rhs.ilog2() + 1);
        let mut divider = rhs.clone() << current_shift;

        current_shift += 1;
        while self >= rhs {
            current_shift -= 1;
            if self >= divider {
                self -= divider.clone();
                quotient += Natural::one() << current_shift;
            }
            divider >>= 1;
        }
        (quotient, self)
    }
}

impl std::ops::Div for Natural {
    type Output = Natural;

    fn div(self, rhs: Self) -> Self::Output {
        self.quot_rem(rhs).0
    }
}

div_variant! { Natural, Natural }
div_unsigned! { Natural }
