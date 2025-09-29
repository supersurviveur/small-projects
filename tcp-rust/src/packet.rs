use crate::{
    checksum::Checksum,
    traits::{Header, Prepare, ToMutable},
    HeaderView, Payload, PayloadView, WriteTo,
};
use std::{
    fmt::Debug,
    io::{self, Write},
    marker::PhantomData,
    ptr::slice_from_raw_parts,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PacketView<'a, H: HeaderView<'a>, C: PayloadView<'a> = &'a [u8]> {
    pub header: H,
    pub payload: C,
    pub phantom: PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Packet<'a, H: Header<'a>, C: Payload<'a> = Vec<u8>> {
    pub header: H,
    pub payload: C,
    pub phantom: PhantomData<&'a ()>,
}

impl<'a, H: Header<'a>, C: Payload<'a>> Prepare for Packet<'a, H, C> {
    #[inline(always)]
    default fn prepare(&mut self) {
        self.header.prepare();
        self.payload.prepare();
    }
}

impl<'a, H: Header<'a>, C: Payload<'a>> WriteTo for Packet<'a, H, C> {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        let mut nbytes = self.header.write_to_inner(writer)?;
        nbytes += self.payload.write_to_inner(writer)?;
        Ok(nbytes)
    }
}

impl<'a, H: Header<'a>, C: Payload<'a>> Packet<'a, H, C> {
    pub fn new(header: H, payload: C) -> Self {
        Self {
            header,
            payload,
            phantom: PhantomData,
        }
    }
}
impl<'a, H: HeaderView<'a>, C: PayloadView<'a>> PacketView<'a, H, C> {
    pub fn new(header: H, payload: C) -> Self {
        Self {
            header,
            payload,
            phantom: PhantomData,
        }
    }
    pub fn from_slice(slice: &'a [u8]) -> Self {
        let header = H::from_slice(slice);
        let header_size = header.size();
        Self::new(header, C::from_slice(&slice[header_size..]))
    }

    pub fn get_header(&self) -> H {
        self.header
    }

    pub fn get_payload(&self) -> C {
        self.payload
    }
}

impl<'a, H: HeaderView<'a>, C: PayloadView<'a>> ToMutable for PacketView<'a, H, C> {
    type MutableType = Packet<'a, H::MutableType, C::MutableType>;

    fn to_mutable(&self) -> Self::MutableType {
        Packet::new(self.header.to_mutable(), self.payload.to_mutable())
    }
}

impl<'a, H: HeaderView<'a>, C: PayloadView<'a>> Debug for PacketView<'a, H, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketView")
            .field("header", &self.get_header())
            .field("data", &self.get_payload())
            .finish()
    }
}

impl<'a, H: HeaderView<'a>, C: PayloadView<'a>> PayloadView<'a> for PacketView<'a, H, C>
where
    Self::MutableType: WriteTo,
{
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::from_slice(slice)
    }
    fn size(&self) -> usize {
        self.header.size() + self.payload.size()
    }

    fn as_bytes(&self) -> &'a [u8] {
        // SAFETY
        // the header and the payload are built from the same slice, so they are contigous in memory
        unsafe { &*slice_from_raw_parts(self.header.as_bytes().as_ptr(), self.size()) }
    }
}
impl<'a, H: Header<'a>, C: Payload<'a>> Payload<'a> for Packet<'a, H, C>
where
    Self: WriteTo,
{
    type ViewType<'b> = PacketView<'b, H::ViewType<'b>, C::ViewType>;

    fn size(&self) -> usize {
        self.header.size() + self.payload.size()
    }
}

impl<'a, H: Header<'a>, C: Payload<'a>> Packet<'a, H, C> {
    pub fn compute_checksum(&mut self) -> Checksum {
        self.header
            .compute_checksum()
            .add_checksum(self.payload.compute_checksum())
    }
}
