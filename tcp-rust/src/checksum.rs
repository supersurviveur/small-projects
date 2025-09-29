use crate::traits::AsArrayUnchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Checksum(pub u64);

impl Checksum {
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn add_8bytes(self, bytes: [u8; 8]) -> Self {
        self.add_u64(u64::from_be_bytes(bytes))
    }
    #[must_use]
    pub fn add_4bytes(self, bytes: [u8; 4]) -> Self {
        self.add_u64(u64::from(u32::from_be_bytes(bytes)))
    }
    #[must_use]
    pub fn add_2bytes(self, bytes: [u8; 2]) -> Self {
        self.add_u64(u64::from(u16::from_be_bytes(bytes)))
    }
    #[must_use]
    pub fn add_byte(self, byte: u8) -> Self {
        self.add_u64(u64::from(byte))
    }

    #[must_use]
    pub fn add_u64(self, value: u64) -> Self {
        let (sum, carry) = self.0.overflowing_add(value);
        let (sum, carry) = sum.overflowing_add(carry as u64);
        Self(sum + carry as u64)
    }
    #[must_use]
    pub fn add_u32(self, value: u16) -> Self {
        self.add_u64(value as u64)
    }
    #[must_use]
    pub fn add_u16(self, value: u16) -> Self {
        self.add_u64(value as u64)
    }
    #[must_use]
    pub fn add_u8(self, value: u16) -> Self {
        self.add_u64(value as u64)
    }

    #[must_use]
    pub fn add_slice(mut self, mut bytes: &[u8]) -> Self {
        macro_rules! add_slice_inner {
            ($n:literal, $fn:ident) => {
                let steps = bytes.len() / $n;
                self = (0..steps).fold(self, |acc, i| {
                    acc.$fn(*unsafe { bytes[i * $n..(i + 1) * $n].as_array_unchecked() })
                });
                bytes = &bytes[steps * $n..];
            };
        }
        add_slice_inner!(8, add_8bytes);
        add_slice_inner!(4, add_4bytes);
        add_slice_inner!(2, add_2bytes);

        if !bytes.is_empty() {
            self = self.add_byte(bytes[0]);
        }

        self
    }
    #[must_use]
    pub fn add_checksum(self, rhs: Self) -> Self {
        self.add_u64(rhs.0)
    }

    #[must_use]
    pub fn to_u16(self) -> u16 {
        let (res, carry) = (self.0 as u16).overflowing_add((self.0 >> 16) as u16);
        let (res, carry) = res.carrying_add((self.0 >> 32) as u16, carry);
        let (res, carry) = res.carrying_add((self.0 >> 48) as u16, carry);
        let (res, carry) = res.overflowing_add(carry as u16);
        res + carry as u16
    }
    #[must_use]
    pub fn ones_complement(self) -> u16 {
        !self.to_u16()
    }
}

impl From<Checksum> for u16 {
    fn from(val: Checksum) -> Self {
        val.to_u16()
    }
}

macro_rules! add_impls {
    ($($ty:ty),*) => {
        $(impl std::ops::Add<$ty> for Checksum {
            type Output = Self;
            fn add(self, rhs: $ty) -> Self::Output {
                self.add_u64(rhs as u64)
            }
        })*
    };
}

add_impls! {
    u8, u16, u32, u64
}
