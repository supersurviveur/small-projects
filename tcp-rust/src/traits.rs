use std::{
    fmt::Debug,
    io::{self, Write},
};

use crate::checksum::Checksum;

pub trait ToMutable {
    type MutableType;
    fn to_mutable(&self) -> Self::MutableType;
}

pub trait Data: Debug + Clone {
    fn size(&self) -> usize;
}

pub trait DataView<'a>:
    Data + Copy + ToMutable<MutableType: DataOwned> + TryFrom<&'a [u8], Error: Debug> + AsRef<[u8]>
{
    fn compute_checksum(&self) -> Checksum {
        Checksum::new().add_slice(self.as_ref())
    }
}

impl<
        'a,
        T: Data
            + Copy
            + ToMutable<MutableType: DataOwned>
            + TryFrom<&'a [u8], Error: Debug>
            + AsRef<[u8]>,
    > DataView<'a> for T
{
}

pub trait DataOwned: Data + WriteTo {
    fn compute_checksum(&mut self) -> Checksum {
        Checksum::new().add_slice(&self.to_bytes().unwrap())
    }
}

impl<T: Data + WriteTo> DataOwned for T {}

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

    fn to_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer)?;
        Ok(buffer)
    }
}

impl WriteTo for u8 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&[*self])?;
        Ok(size_of::<Self>())
    }
}

impl WriteTo for u16 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(size_of::<Self>())
    }
}

impl WriteTo for u32 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(size_of::<Self>())
    }
}

impl WriteTo for i32 {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(size_of::<Self>())
    }
}

impl WriteTo for &[u8] {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self)?;
        Ok(self.len())
    }
}

impl WriteTo for &str {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self.as_bytes())?;
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
        Ok(size_of::<Self>())
    }
}

impl Prepare for u8 {}
impl Prepare for u16 {}
impl Prepare for u32 {}
impl Prepare for i32 {}
impl Prepare for &str {}
impl Prepare for &[u8] {}
impl Prepare for Vec<u8> {}
impl<const N: usize> Prepare for [u8; N] {}

impl ToMutable for &[u8] {
    type MutableType = Vec<u8>;

    fn to_mutable(&self) -> Self::MutableType {
        self.to_vec()
    }
}

impl Data for &[u8] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Data for Vec<u8> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Data for &str {
    fn size(&self) -> usize {
        self.len()
    }
}
