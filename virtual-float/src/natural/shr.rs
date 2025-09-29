use apars_macros::{shr_signed, shr_unsigned, shr_variant};

use crate::natural::Natural;

impl std::ops::Shr<usize> for Natural {
    type Output = Natural;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let big_steps = rhs / 64;
        let small_steps = rhs % 64;
        self.inner = self.inner.into_iter().skip(big_steps).collect();

        let mut overflow = 0;
        for i in (0..self.chunk_count()).rev() {
            let tmp = self.inner[i];
            self.inner[i] >>= small_steps;
            self.inner[i] |= overflow;
            overflow = tmp.unbounded_shl((64 - small_steps) as u32);
        }

        self.fit();

        self
    }
}

shr_variant! { Natural, usize }
shr_unsigned! { Natural, u8, u16, u32, u64 }
shr_signed! { Natural }
