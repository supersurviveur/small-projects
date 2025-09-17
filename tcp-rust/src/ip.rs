use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};

use crate::{
    traits::{Header, HeaderView, ToMutable, WriteTo},
    AsArrayUnchecked,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IpProtocol {
    Icmp = 1,
    Igmp = 2,
    Tcp = 6,
    Udp = 17,
    IpV6Encap = 41,
    Ospf = 89,
    Sctp = 132,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpV4Addr(pub u32);

impl From<u32> for IpV4Addr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPV4HeaderView<'a> {
    content: &'a [u8],
}

impl ToMutable for IPV4HeaderView<'_> {
    type MutableType = IPV4Header;

    fn to_mutable(&self) -> Self::MutableType {
        IPV4Header {
            version: self.get_version(),
            ihl: self.get_ihl(),
            dscp: self.get_dscp(),
            ecn: self.get_ecn(),
            total_length: self.get_total_length(),
            identification: self.get_identification(),
            flags: self.get_flags(),
            fragment_offset: self.get_fragment_offset(),
            ttl: self.get_ttl(),
            protocol: self.get_protocol(),
            header_checksum: self.get_header_checksum(),
            source_address: self.get_source_address(),
            destination_address: self.get_destination_address(),
        }
    }
}

impl<'a> HeaderView<'a> for IPV4HeaderView<'a> {
    fn from_slice(slice: &'a [u8]) -> Self {
        let ihl = slice[0] & 0xF;
        Self {
            content: &slice[..20],
        }
    }

    fn size(&self) -> usize {
        self.content.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPV4Header {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: IpProtocol,
    pub header_checksum: u16,
    pub source_address: IpV4Addr,
    pub destination_address: IpV4Addr,
}

impl WriteTo for IPV4Header {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.version << 4 | self.ihl, self.dscp << 2 | self.ecn])?;
        writer.write_all(&u16::to_be_bytes(self.total_length))?;
        writer.write_all(&u16::to_be_bytes(self.identification))?;
        writer.write_all(&[
            self.flags << 5 | (self.fragment_offset >> 8) as u8,
            (self.fragment_offset & 0xFF) as u8,
            self.ttl,
            self.protocol as u8,
        ])?;
        writer.write_all(&u16::to_be_bytes(self.header_checksum))?;
        writer.write_all(&u32::to_be_bytes(self.source_address.0))?;
        writer.write_all(&u32::to_be_bytes(self.destination_address.0))?;
        Ok(())
    }
}

impl<'a> Header<'a> for IPV4Header {
    type ViewType = IPV4HeaderView<'a>;

    fn size(&self) -> usize {
        self.ihl as usize * 4
    }
}

impl<'a> IPV4HeaderView<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        self.content
    }
    pub fn get_version(&self) -> u8 {
        self.content[0] >> 4
    }
    pub fn get_ihl(&self) -> u8 {
        self.content[0] & 0xF
    }
    pub fn get_dscp(&self) -> u8 {
        self.content[1] >> 2
    }
    pub fn get_ecn(&self) -> u8 {
        self.content[1] & 0b111
    }
    pub fn get_total_length(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[2..=3].as_array_unchecked() })
    }
    pub fn get_identification(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[4..=5].as_array_unchecked() })
    }
    pub fn get_flags(&self) -> u8 {
        (self.content[6] >> 5)
    }
    pub fn get_fragment_offset(&self) -> u16 {
        u16::from_be((((self.content[6] & 0x1F) as u16) << 8) + self.content[7] as u16)
    }
    pub fn get_ttl(&self) -> u8 {
        self.content[8]
    }
    pub fn get_protocol(&self) -> IpProtocol {
        unsafe { core::mem::transmute::<u8, IpProtocol>(self.content[9]) }
    }
    pub fn get_header_checksum(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[10..=11].as_array_unchecked() })
    }
    pub fn get_source_address(&self) -> IpV4Addr {
        u32::from_be_bytes(*unsafe { self.content[12..=15].as_array_unchecked() }).into()
    }
    pub fn get_destination_address(&self) -> IpV4Addr {
        u32::from_be_bytes(*unsafe { self.content[16..=19].as_array_unchecked() }).into()
    }
}
impl Debug for IpV4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            (self.0 >> 24) & 0xFF,
            (self.0 >> 16) & 0xFF,
            (self.0 >> 8) & 0xFF,
            self.0 & 0xFF,
        )
    }
}

impl Display for IpV4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for IPV4HeaderView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IPV4HeaderView")
            .field("version", &self.get_version())
            .field("ihl", &self.get_ihl())
            .field("dscp", &self.get_dscp())
            .field("ecn", &self.get_ecn())
            .field("total_length", &self.get_total_length())
            .field("identification", &self.get_identification())
            .field("flags", &self.get_flags())
            .field("fragment_offset", &self.get_fragment_offset())
            .field("ttl", &self.get_ttl())
            .field("protocol", &self.get_protocol())
            .field("header_checksum", &self.get_header_checksum())
            .field("source_address", &self.get_source_address())
            .field("destination_address", &self.get_destination_address())
            .finish()
    }
}

impl IPV4Header {
    pub fn compute_checksum(&self) -> u16 {
        let mut sum = ((self.version as u16) << 12)
            + ((self.ihl as u16) << 8)
            + ((self.dscp as u16) << 2)
            + self.ecn as u16;
        let mut carry = false;
        (sum, carry) = sum.carrying_add(self.total_length, carry);
        (sum, carry) = sum.carrying_add(self.identification, carry);
        (sum, carry) = sum.carrying_add(((self.flags as u16) << 13) + self.fragment_offset, carry);
        (sum, carry) = sum.carrying_add(((self.ttl as u16) << 8) + self.protocol as u16, carry);
        (sum, carry) = sum.carrying_add(self.header_checksum, carry);
        (sum, carry) = sum.carrying_add((self.source_address.0 >> 16) as u16, carry);
        (sum, carry) = sum.carrying_add((self.source_address.0 & 0xFFFF) as u16, carry);
        (sum, carry) = sum.carrying_add((self.destination_address.0 >> 16) as u16, carry);
        (sum, carry) = sum.carrying_add((self.destination_address.0 & 0xFFFF) as u16, carry);
        (sum, carry) = sum.overflowing_add(carry as u16);
        sum += carry as u16;
        sum
    }
    pub fn set_checksum(&mut self) {
        self.header_checksum = 0;
        self.header_checksum = !self.compute_checksum();
    }
}
