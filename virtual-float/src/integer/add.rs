use apars_macros::{add_signed, add_unsigned, add_variant};

use crate::integer::Integer;

impl std::ops::Add for Integer {
    type Output = Integer;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (false, false) | (true, true) => {
                self.abs += rhs.abs;
                self
            }
            (true, false) | (false, true) if self.abs <= rhs.abs => {
                rhs.abs -= self.abs;
                rhs.sign &= !rhs.abs.is_zero();
                rhs
            }
            (true, false) | (false, true) if self.abs > rhs.abs => {
                self.abs -= rhs.abs;
                self.sign &= !self.abs.is_zero();
                self
            }
            _ => unreachable!(),
        }
    }
}

add_variant! { Integer, Integer }
add_unsigned! { Integer }
add_signed! { Integer }
