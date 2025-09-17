use std::{
    fmt::Debug,
    io::{self, Write},
};

use crate::{
    traits::{Header, HeaderView, ToMutable, WriteTo},
    AsArrayUnchecked,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ICMPHeaderView<'a> {
    content: &'a [u8],
}

impl<'a> ICMPHeaderView<'a> {
    pub fn get_message_type(&self) -> u8 {
        self.content[0]
    }
    pub fn get_code(&self) -> u8 {
        self.content[1]
    }
    pub fn get_checksum(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[2..=3].as_array_unchecked() })
    }

    pub fn to_mutable(self) -> ICMPHeader {
        ICMPHeader {
            message_type: self.get_message_type(),
            code: self.get_code(),
            checksum: self.get_checksum(),
        }
    }
}

impl Debug for ICMPHeaderView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ICMPHeaderView")
            .field("message_type", &self.get_message_type())
            .field("code", &self.get_code())
            .field("checksum", &self.get_checksum())
            .finish()
    }
}
impl<'a> ToMutable for ICMPHeaderView<'a> {
    type MutableType = ICMPHeader;

    fn to_mutable(&self) -> Self::MutableType {
        Self::MutableType {
            message_type: self.get_message_type(),
            code: self.get_code(),
            checksum: self.get_checksum(),
        }
    }
}

impl<'a> HeaderView<'a> for ICMPHeaderView<'a> {
    fn from_slice(slice: &'a [u8]) -> Self {
        Self { content: slice }
    }

    fn size(&self) -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ICMPHeader {
    pub message_type: u8,
    pub code: u8,
    pub checksum: u16,
}

impl WriteTo for ICMPHeader {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.message_type])?;
        writer.write_all(&[self.code])?;
        writer.write_all(&self.checksum.to_be_bytes())?;
        Ok(())
    }
}

impl<'a> Header<'a> for ICMPHeader {
    type ViewType = ICMPHeaderView<'a>;

    fn size(&self) -> usize {
        4
    }
    // fn compute_checksum(&self) -> u16 {
    //     let sum = ((self.message_type as u16) << 8) + self.code as u16;
    //     let (sum, carry) = sum.overflowing_add(self.checksum);
    //     sum + carry as u16
    // }
}
