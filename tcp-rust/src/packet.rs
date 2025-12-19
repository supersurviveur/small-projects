use crate::{
    checksum::Checksum,
    traits::{Data, DataOwned, DataView, Prepare, ToMutable, WriteTo},
};
use std::{
    fmt::Debug,
    io::{self, Write},
    marker::PhantomData,
    ptr::slice_from_raw_parts,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PacketView<'a, H: DataView<'a>, C: DataView<'a> = &'a [u8]> {
    pub header: H,
    pub payload: C,
    pub phantom: PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Packet<H: DataOwned, C: DataOwned = Vec<u8>> {
    pub header: H,
    pub payload: C,
}

impl<H: DataOwned, C: DataOwned> Prepare for Packet<H, C> {
    #[inline(always)]
    default fn prepare(&mut self) {
        self.header.prepare();
        self.payload.prepare();
    }
}

impl<H: DataOwned, C: DataOwned> WriteTo for Packet<H, C> {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        let mut nbytes = self.header.write_to_inner(writer)?;
        nbytes += self.payload.write_to_inner(writer)?;
        Ok(nbytes)
    }
}

impl<H: DataOwned, C: DataOwned> Packet<H, C> {
    pub fn new(header: H, payload: C) -> Self {
        Self { header, payload }
    }
}
impl<'a, H: DataView<'a>, C: DataView<'a>> PacketView<'a, H, C> {
    pub fn new(header: H, payload: C) -> Self {
        Self {
            header,
            payload,
            phantom: PhantomData,
        }
    }

    pub fn get_header(&self) -> H {
        self.header
    }

    pub fn get_payload(&self) -> C {
        self.payload
    }
}

impl<'a, H: DataView<'a>, C: DataView<'a>> ToMutable for PacketView<'a, H, C> {
    type MutableType = Packet<H::MutableType, C::MutableType>;

    fn to_mutable(&self) -> Self::MutableType {
        Packet::new(self.header.to_mutable(), self.payload.to_mutable())
    }
}

impl<'a, H: DataView<'a>, C: DataView<'a>> Debug for PacketView<'a, H, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketView")
            .field("header", &self.get_header())
            .field("data", &self.get_payload())
            .finish()
    }
}

impl<'a, H: DataView<'a>, C: DataView<'a, Error = H::Error>> TryFrom<&'a [u8]>
    for PacketView<'a, H, C>
{
    type Error = H::Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let header = H::try_from(value)?;
        let header_size = header.size();
        Ok(Self::new(header, C::try_from(&value[header_size..])?))
    }
}

impl<'a, H: DataView<'a>, C: DataView<'a>> AsRef<[u8]> for PacketView<'a, H, C> {
    fn as_ref(&self) -> &[u8] {
        // SAFETY
        // the header and the payload are built from the same slice, so they are contigous in memory
        unsafe { &*slice_from_raw_parts(self.header.as_ref().as_ptr(), self.size()) }
    }
}
impl<'a, H: DataView<'a>, C: DataView<'a>> Data for PacketView<'a, H, C> {
    fn size(&self) -> usize {
        self.header.size() + self.payload.size()
    }
}

impl<H: DataOwned, C: DataOwned> Data for Packet<H, C> {
    fn size(&self) -> usize {
        self.header.size() + self.payload.size()
    }
}

impl<H: DataOwned, C: DataOwned> Packet<H, C> {
    pub fn compute_checksum(&mut self) -> Checksum {
        self.header
            .compute_checksum()
            .add_checksum(self.payload.compute_checksum())
    }
}
