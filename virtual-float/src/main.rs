#[allow(unused_imports)]
use std::{
    fmt::{Binary, Debug},
    ops::Add,
};

use virtual_float::integer::Integer;

// pub struct Virtualf32(pub(crate) u32);

// impl Virtualf32 {
//     pub fn from_binary(sign: bool, exponent: u32, mantissa: u32) -> Self {
//         Self(mantissa | (exponent << 23) | ((sign as u32) << 31))
//     }
//     pub fn from_f32(value: f32) -> Self {
//         Self(f32::to_bits(value))
//     }

//     fn get_sign(&self) -> bool {
//         (self.0 >> 31) == 1
//     }
//     fn get_exponent(&self) -> u32 {
//         (self.0 >> 23) & 0xFF
//     }
//     fn get_mantissa(&self) -> u32 {
//         self.0 & 0x7FFFFF
//     }
// }

// impl Binary for Virtualf32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:032b}", self.0)
//     }
// }
// impl Debug for Virtualf32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", f32::from_bits(self.0))
//     }
// }

// impl Add for Virtualf32 {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         let mut s1 = self.get_sign();
//         let mut e1 = self.get_exponent();
//         let mut m1 = self.get_mantissa();
//         let mut s2 = rhs.get_sign();
//         let mut e2 = rhs.get_exponent();
//         let mut m2 = rhs.get_mantissa();
//         if e1 == e2 {
//             let mut m = m1 + m2;
//             let e = e1 + 1;
//             m >>= 1;
//             Virtualf32::from_binary(false, e, m)
//         } else {
//             if e1 < e2 {
//                 (e1, e2) = (e2, e1);
//                 (m1, m2) = (m2, m1);
//                 // (s1, s2) = (s2, s1);
//             }
//             let diff_e = e1 - e2;

//             let mut m = m1 + ((0x800000 + m2) >> diff_e);

//             let mut e = e1;
//             if m & 0x800000 != 0 {
//                 m &= 0x7FFFFF;
//                 m >>= 1;
//                 e += 1;
//             }

//             Virtualf32::from_binary(false, e, m)
//         }
//     }
// }

fn main() {
    // let a = Virtualf32::from_f32(0.25);
    // let b = Virtualf32::from_f32(6.25);
    // println!("{a:b}");
    // println!("{a:?}");
    // println!("{b:?}");
    // println!("{:?}", a + b);

    let a = Integer::from(-21i64);
    let b = Integer::from(-3i64);
    let c = Integer::from(u64::MAX);
    println!("{}", a.clone());
    println!("{}", b.clone());
    println!("{}", a.clone() + b.clone());
    println!("{}", a.clone() - b.clone());
    println!("{}", b.clone() - a.clone());
    println!("{}", c.clone() + a.clone());
    println!("{:?}", a.clone().quot_rem(b.clone()));
    println!("{}", a.clone() >> 1);
}
