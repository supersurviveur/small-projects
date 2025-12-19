use std::{
    fmt::Debug,
    io::{self, Write},
};

use crate::{
    packet::{Packet, PacketView},
    traits::{AsArrayUnchecked, Data, DataOwned, Prepare, ToMutable, WriteTo},
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

impl<'a> From<&'a [u8]> for ICMPHeaderView<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { content: value }
    }
}

impl<'a> AsRef<[u8]> for ICMPHeaderView<'a> {
    fn as_ref(&self) -> &[u8] {
        self.content
    }
}

impl Data for ICMPHeaderView<'_> {
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

impl Prepare for ICMPHeader {}
impl WriteTo for ICMPHeader {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&[self.message_type])?;
        writer.write_all(&[self.code])?;
        writer.write_all(&self.checksum.to_be_bytes())?;
        Ok(self.size())
    }
}

impl Data for ICMPHeader {
    fn size(&self) -> usize {
        4
    }
}

pub type ICMPPacket<C = Vec<u8>> = Packet<ICMPHeader, C>;
pub type ICMPPacketView<'a, C = &'a [u8]> = PacketView<'a, ICMPHeaderView<'a>, C>;

impl<C: DataOwned> Prepare for ICMPPacket<C> {
    fn prepare(&mut self) {
        self.header.prepare();
        self.payload.prepare();
        self.header.checksum = 0;
        self.header.checksum = self.compute_checksum().ones_complement();
    }
}
