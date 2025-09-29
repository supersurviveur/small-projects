pub mod add;
pub mod div;
pub mod mul;
pub mod ord;
pub mod shl;
pub mod shr;
pub mod sub;

use std::{
    fmt::{Binary, Debug, Display},
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
                Self::new(
                    if value == 0 {
                        vec![]
                    } else {
                        vec![value as u64]
                    }
                )
            }
        })*
    };
}

from_impl! {
    u8, u16, u32, u64, usize
}

#[derive(Debug)]
pub struct TryFromIntError;

#[derive(Debug)]
pub struct TryFromNaturalError;

macro_rules! try_from_impl {
    ($($type:ident),*) => {
        $(impl TryFrom<$type> for Natural {
            type Error = TryFromIntError;
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if value >= 0 {
                    Ok(Self::from(value.unsigned_abs()))
                } else {
                    Err(TryFromIntError)
                }
            }
        })*
    };
}

try_from_impl! {
    i8, i16, i32, i64, isize
}

macro_rules! try_from_natural_impl {
    ($($type:ident),*) => {
        $(impl TryFrom<Natural> for $type {
            type Error = TryFromNaturalError;

            fn try_from(value: Natural) -> Result<Self, Self::Error> {
                if value.is_zero() {
                    Ok(0)
                } else if value.chunk_count() == 1 {
                    $type::try_from(value.inner[0]).map_err(|_| TryFromNaturalError)
                } else {
                    Err(TryFromNaturalError)
                }
            }
        })*
    };
}

try_from_natural_impl! {
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
}

impl Binary for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        for (i, data) in self.inner.iter().rev().enumerate() {
            // Complete with 0 only if it's not the first block
            if i == 0 {
                write!(f, "{:b}", data)?;
            } else {
                write!(f, "{:064b}", data)?;
            }
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
            result *= 10u8;
            if c.is_numeric() && c.is_ascii_alphanumeric() {
                result += (c as u8) - b'0';
            } else {
                return Err(ParseNaturalError);
            }
        }
        Ok(result)
    }
}
