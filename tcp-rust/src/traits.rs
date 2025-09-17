use std::{
    fmt::Debug,
    io::{self, Write},
};

pub trait ToMutable {
    type MutableType;
    fn to_mutable(&self) -> Self::MutableType;
}

pub trait HeaderView<'a>: Debug + Copy + ToMutable<MutableType: Header<'a>> {
    fn from_slice(slice: &'a [u8]) -> Self;
    fn size(&self) -> usize;
}

pub trait Header<'a>: Debug + Clone + WriteTo {
    type ViewType: HeaderView<'a> + ToMutable<MutableType = Self>;
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::ViewType::from_slice(slice).to_mutable()
    }
    fn size(&self) -> usize;
    fn compute_checksum(&self) -> u16 {
        let bytes = self.to_bytes().unwrap();
        let (chunks, end) = bytes.as_chunks::<2>();
        if !end.is_empty() {
            panic!()
        }
        chunks.iter().fold(0, |acc, b| {
            let (acc, carry) = acc.overflowing_add(((b[0] as u16) << 8) + b[1] as u16);
            acc + carry as u16
        })
    }
}
pub trait PayloadView<'a>: Debug + Copy + ToMutable<MutableType: Payload<'a>> {
    fn from_slice(slice: &'a [u8]) -> Self;
}

pub trait Payload<'a>: Debug + Clone + WriteTo {
    type ViewType: PayloadView<'a> + ToMutable<MutableType = Self>;
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::ViewType::from_slice(slice).to_mutable()
    }
    fn size(&self) -> usize;
    fn compute_checksum(&self) -> u16 {
        let bytes = self.to_bytes().unwrap();
        let (chunks, end) = bytes.as_chunks::<2>();

        let tmp: u16 = chunks.iter().fold(0, |acc, b| {
            let (acc, carry) = acc.overflowing_add(((b[0] as u16) << 8) + b[1] as u16);
            acc + carry as u16
        });
        let (tmp, carry) = tmp.overflowing_add(if !end.is_empty() {
            (end[0] as u16) << 8
        } else {
            0
        });
        tmp + carry as u16
    }
}

pub trait WriteTo {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()>;

    fn write_into<W: Write>(self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
    {
        self.write_to(writer)
    }

    fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer)?;
        Ok(buffer)
    }
}

impl WriteTo for u8 {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[*self])
    }
}

impl WriteTo for u16 {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl WriteTo for u32 {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl WriteTo for i32 {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl WriteTo for &[u8] {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self)
    }
}

impl WriteTo for Vec<u8> {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self)
    }
}

impl<const N: usize> WriteTo for [u8; N] {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self)
    }
}
