use std::{
    cmp::Ordering,
    fmt::{Binary, Debug, Display},
    ops::{Add, Div, Mul, Shl, Shr, Sub},
    str::FromStr,
};

use crate::natural::{Natural, ParseNaturalError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Integer {
    pub sign: bool,
    pub abs: Natural,
}

macro_rules! from_impl_unsigned {
    ($($type:ident),*) => {
        $(impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                Self {
                    sign: false,
                    abs: Natural::from(value),
                }
            }
        })*
    };
}
macro_rules! from_impl_signed {
    ($($type:ident),*) => {
        $(impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                let sign = value < 0;
                Self {
                    sign,
                    abs: Natural::from(value.abs().cast_unsigned()),
                }
            }
        })*
    };
}

from_impl_unsigned! {
    u8, u16, u32, u64
}
from_impl_signed! {
    i8, i16, i32, i64
}

#[derive(Debug)]
pub struct TryFromIntegerError;

impl TryFrom<Integer> for u8 {
    type Error = TryFromIntegerError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        if value.sign {
            Err(TryFromIntegerError)
        } else {
            value.abs.try_into().map_err(|_| TryFromIntegerError)
        }
    }
}

impl Binary for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign {
            write!(f, "-")?;
        }
        Binary::fmt(&self.abs, f)
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign {
            write!(f, "-")?;
        }
        Display::fmt(&self.abs, f)
    }
}

impl Integer {
    pub fn zero() -> Self {
        Self::new(false, Natural::zero())
    }
    pub fn is_zero(&self) -> bool {
        self.abs.is_zero()
    }
    pub fn one() -> Self {
        Self::new(false, Natural::one())
    }
    pub fn new(sign: bool, abs: Natural) -> Self {
        Self { sign, abs }
    }

    pub fn ilog2(&self) -> usize {
        if *self < Self::zero() {
            panic!("todo")
        }
        self.abs.ilog2()
    }

    pub fn quot_rem(self, rhs: Self) -> (Integer, Natural) {
        let (quotient, rem) = self.abs.quot_rem(rhs.abs.clone());
        if self.sign && !rem.is_zero() {
            (
                Integer::new(self.sign ^ rhs.sign, quotient + 1),
                rhs.abs - rem,
            )
        } else {
            (Integer::new(self.sign ^ rhs.sign, quotient), rem)
        }
    }
}

impl Add for Integer {
    type Output = Integer;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (false, false) | (true, true) => Integer::new(self.sign, self.abs + rhs.abs),
            (true, false) | (false, true) if self.abs <= rhs.abs => {
                let abs = rhs.abs - self.abs;
                Integer::new(!self.sign & !abs.is_zero(), abs)
            }
            (true, false) | (false, true) if self.abs > rhs.abs => {
                let abs = self.abs - rhs.abs;
                Integer::new(self.sign & !abs.is_zero(), abs)
            }
            _ => unreachable!(),
        }
    }
}
impl Sub for Integer {
    type Output = Integer;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (true, false) | (false, true) => Integer::new(self.sign, self.abs + rhs.abs),
            (false, false) | (true, true) if self.abs >= rhs.abs => {
                let abs = self.abs - rhs.abs;
                Integer::new(self.sign & !abs.is_zero(), abs)
            }
            (false, false) | (true, true) if self.abs < rhs.abs => {
                let abs = rhs.abs - self.abs;
                Integer::new(!self.sign & !abs.is_zero(), abs)
            }
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (false, false) => self.abs.cmp(&other.abs),
            (true, true) => self.abs.cmp(&other.abs).reverse(),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        }
    }
}

impl Div for Integer {
    type Output = Integer;

    fn div(self, rhs: Self) -> Self::Output {
        self.quot_rem(rhs).0
    }
}

impl Mul for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.sign ^= rhs.sign;
        self.abs = self.abs * rhs.abs;
        self
    }
}

impl Shl<usize> for Integer {
    type Output = Integer;

    fn shl(mut self, rhs: usize) -> Self::Output {
        self.abs = self.abs.shl(rhs);
        self
    }
}
impl Shr<usize> for Integer {
    type Output = Integer;

    fn shr(mut self, rhs: usize) -> Self::Output {
        let tmp = self.abs.clone();
        self.abs = self.abs.shr(rhs);
        if !self.abs.is_zero() && self.abs.clone().shl(rhs) != tmp && self.sign {
            self.abs = self.abs.shl(1);
        }
        if self.abs.is_zero() {
            self.sign = false;
        }
        self
    }
}

impl FromStr for Integer {
    type Err = ParseNaturalError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let sign = s.starts_with('-');
        if sign {
            s = &s[1..]
        }
        let abs = Natural::from_str(s)?;
        Ok(Integer::new(sign && !abs.is_zero(), abs))
    }
}
