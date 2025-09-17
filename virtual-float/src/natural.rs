use std::{
    cmp::Ordering,
    fmt::{Binary, Debug, Display},
    ops::{Add, Div, Mul, Shl, Shr, Sub},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Natural {
    pub inner: Vec<u64>,
}

macro_rules! from_impl {
    ($($type:ident),*) => {
        $(impl From<$type> for Natural {
            fn from(value: $type) -> Self {
                Self {
                    inner: {
                        if value == 0 {
                            vec![]
                        } else {
                            vec![value as u64]
                        }
                    },
                }
            }
        })*
    };
}

from_impl! {
    u8, u16, u32, u64
}

#[derive(Debug)]
pub struct TryFromNaturalError;

impl TryFrom<Natural> for u8 {
    type Error = TryFromNaturalError;

    fn try_from(value: Natural) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Ok(0)
        } else if value.chunk_count() == 1 {
            u8::try_from(value.inner[0]).map_err(|_| TryFromNaturalError)
        } else {
            Err(TryFromNaturalError)
        }
    }
}

impl Binary for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        for data in self.inner.iter().rev() {
            write!(f, "{:064b}", data)?;
        }
        Ok(())
    }
}
impl Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut current = self.clone();
        let mut rem;
        let mut res = String::new();

        while current != Natural::zero() {
            (current, rem) = current.quot_rem(Natural::from(10u8));
            res.push((b'0' + u8::try_from(rem).unwrap()) as char);
        }
        for i in res.bytes().rev() {
            write!(f, "{}", i as char)?;
        }
        Ok(())
    }
}

impl Natural {
    pub fn zero() -> Self {
        Self::new(Vec::new())
    }
    pub fn is_zero(&self) -> bool {
        self.chunk_count() == 0
    }
    pub fn one() -> Self {
        Self::new(vec![1])
    }
    pub fn new(inner: Vec<u64>) -> Self {
        Self { inner }
    }
    fn chunk_count(&self) -> usize {
        self.inner.len()
    }
    fn fit(&mut self) {
        while self.inner.last().is_some_and(|x| *x == 0) {
            self.inner.pop();
        }
    }

    pub fn ilog2(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        (self.inner.len() - 1) * 64 + self.inner[self.inner.len() - 1].ilog2() as usize
    }

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
                self = self - divider.clone();
                quotient = quotient + (Natural::one() << current_shift);
            }
            divider = divider >> 1;
        }
        (quotient, self)
    }
}

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
impl Add<u8> for Natural {
    type Output = Natural;

    fn add(self, rhs: u8) -> Self::Output {
        self + Natural::from(rhs)
    }
}

impl Sub for Natural {
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

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.chunk_count() > other.chunk_count() {
            Ordering::Greater
        } else if self.chunk_count() < other.chunk_count() {
            Ordering::Less
        } else {
            self.inner.iter().rev().cmp(other.inner.iter().rev())
        }
    }
}

impl Div for Natural {
    type Output = Natural;

    fn div(self, rhs: Self) -> Self::Output {
        self.quot_rem(rhs).0
    }
}

impl Mul for Natural {
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
                multiplier = multiplier << (i - prev_shift);
                result = result + multiplier.clone();
                prev_shift = i;
            }
        }
        result
    }
}

impl Shl<usize> for Natural {
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
impl Shr<usize> for Natural {
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

#[derive(Debug)]
pub struct ParseNaturalError;

impl FromStr for Natural {
    type Err = ParseNaturalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::zero();
        if s.is_empty() {
            return Err(ParseNaturalError);
        }
        for c in s.chars() {
            result = result * 10u8.into();
            if c.is_numeric() && c.is_ascii_alphanumeric() {
                result = result + Natural::from((c as u8) - b'0');
            } else {
                return Err(ParseNaturalError);
            }
        }
        Ok(result)
    }
}
