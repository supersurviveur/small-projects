use std::{
    fmt::Debug,
    io::{self, Write},
};

use crate::checksum::Checksum;

pub trait ToMutable {
    type MutableType;
    fn to_mutable(&self) -> Self::MutableType;
}

pub trait HeaderView<'a>: Debug + Copy + ToMutable<MutableType: Header<'a>> {
    fn from_slice(slice: &'a [u8]) -> Self;
    fn as_bytes(&self) -> &'a [u8];
    fn size(&self) -> usize;

    fn compute_checksum(&self) -> Checksum {
        Checksum::new().add_slice(self.as_bytes())
    }
}

pub trait Header<'a>: Debug + Clone + WriteTo {
    type ViewType<'b>: HeaderView<'b> + ToMutable<MutableType = Self>;
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::ViewType::from_slice(slice).to_mutable()
    }
    fn size(&self) -> usize;
    fn compute_checksum(&mut self) -> Checksum {
        Checksum::new().add_slice(&self.to_bytes().unwrap())
    }
}
pub trait PayloadView<'a>: Debug + Copy + ToMutable<MutableType: Payload<'a>> {
    fn from_slice(slice: &'a [u8]) -> Self;
    fn as_bytes(&self) -> &'a [u8];
    fn size(&self) -> usize;

    fn compute_checksum(&self) -> Checksum {
        Checksum::new().add_slice(self.as_bytes())
    }
}

pub trait Payload<'a>: Debug + Clone + WriteTo {
    type ViewType: PayloadView<'a> + ToMutable<MutableType = Self>;
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::ViewType::from_slice(slice).to_mutable()
    }
    fn size(&self) -> usize;
    fn compute_checksum(&mut self) -> Checksum {
        Checksum::new().add_slice(&self.to_bytes().unwrap())
    }
}

pub trait AsArrayUnchecked<T> {
    /// # Safety
    /// The length of self must be N
    unsafe fn as_array_unchecked<const N: usize>(&self) -> &[T; N];
}

impl<T> AsArrayUnchecked<T> for [T] {
    unsafe fn as_array_unchecked<const N: usize>(&self) -> &[T; N] {
        unsafe { &*(self.as_ptr() as *const _) }
    }
}

pub trait Prepare {
    fn prepare(&mut self) {}
}

pub trait WriteTo: Prepare {
    fn write_to<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        self.prepare();
        self.write_to_inner(writer)
    }

    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize>;

    fn write_into<W: Write>(mut self, writer: &mut W) -> io::Result<usize>
    where
        Self: Sized,
    {
        self.write_to(writer)
    }

    fn to_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer)?;
        Ok(buffer)
    }
}

impl WriteTo for u8 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&[*self])?;
        Ok(1)
    }
}

impl WriteTo for u16 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(2)
    }
}

impl WriteTo for u32 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(4)
    }
}

impl WriteTo for i32 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(4)
    }
}

impl WriteTo for &[u8] {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self)?;
        Ok(self.len())
    }
}

impl WriteTo for Vec<u8> {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self)?;
        Ok(self.len())
    }
}

impl<const N: usize> WriteTo for [u8; N] {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self)?;
        Ok(N)
    }
}

impl Prepare for u8 {}
impl Prepare for u16 {}
impl Prepare for u32 {}
impl Prepare for i32 {}
impl Prepare for &[u8] {}
impl Prepare for Vec<u8> {}
impl<const N: usize> Prepare for [u8; N] {}

impl ToMutable for &[u8] {
    type MutableType = Vec<u8>;

    fn to_mutable(&self) -> Self::MutableType {
        self.to_vec()
    }
}

impl<'a> PayloadView<'a> for &'a [u8] {
    fn from_slice(slice: &'a [u8]) -> Self {
        slice
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn as_bytes(&self) -> &'a [u8] {
        self
    }
}

impl<'a> Payload<'a> for Vec<u8> {
    type ViewType = &'a [u8];

    fn size(&self) -> usize {
        self.len()
    }
}
