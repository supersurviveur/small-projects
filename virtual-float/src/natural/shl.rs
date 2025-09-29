use apars_macros::{shl_signed, shl_unsigned, shl_variant};

use crate::natural::Natural;

impl std::ops::Shl<usize> for Natural {
    type Output = Natural;

    fn shl(mut self, rhs: usize) -> Self::Output {
        let big_steps = rhs / 64;
        let small_steps = rhs % 64;
        for _ in 0..big_steps {
            self.inner.insert(0, 0);
        }

        let mut overflow = 0;
        for i in 0..self.chunk_count() {
            let tmp = self.inner[i];
            self.inner[i] <<= small_steps;
            self.inner[i] |= overflow;
            overflow = tmp.unbounded_shr((64 - small_steps) as u32);
        }
        if overflow != 0 {
            self.inner.push(overflow);
        }

        self
    }
}

shl_variant! { Natural, usize }
shl_unsigned! { Natural, u8, u16, u32, u64 }
shl_signed! { Natural }
