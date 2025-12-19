use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::{self, Write},
    iter::Map,
    str::Split,
};

use crate::{
    packet::{Packet, PacketView},
    traits::{Data, Prepare, ToMutable, WriteTo},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HTTPRequestHeaderView<'a> {
    content: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HTTPMethod {
    #[default]
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HTTPHeaderView<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HTTPVersion {
    major: u8,
    minor: u8,
}

impl Display for HTTPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}

impl HTTPMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HTTPMethod::Get => "GET",
            HTTPMethod::Post => todo!(),
        }
    }
}

impl ToMutable for HTTPRequestHeaderView<'_> {
    type MutableType = HTTPRequestHeader;

    fn to_mutable(&self) -> Self::MutableType {
        HTTPRequestHeader {
            method: self.get_method(),
            path: self.get_path().to_string(),
            version: self.get_version(),
            headers: self
                .get_headers_parsed()
                .map(|header| (header.key.to_string(), header.value.to_string()))
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[derive(Debug)]
pub struct ParseHTTPError;

impl<'a> TryFrom<&'a [u8]> for HTTPRequestHeaderView<'a> {
    type Error = ParseHTTPError;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let length = value
            .windows(4)
            .enumerate()
            .find_map(|(i, window)| {
                if window[0..2] == window[2..4] && window[0..2] == *b"\r\n" {
                    Some(i)
                } else {
                    None
                }
            })
            .ok_or(ParseHTTPError)?;

        Ok(Self {
            content: str::from_utf8(&value[..length]).map_err(|_| ParseHTTPError)?,
        })
    }
}

impl<'a> AsRef<[u8]> for HTTPRequestHeaderView<'a> {
    fn as_ref(&self) -> &[u8] {
        self.content.as_bytes()
    }
}

impl Data for HTTPRequestHeaderView<'_> {
    fn size(&self) -> usize {
        self.content.len()
    }
}

impl<'a> HTTPRequestHeaderView<'a> {
    fn get_first_line(&self) -> &str {
        self.content.split_once("\r\n").unwrap().0
    }
    pub fn get_headers_raw(&self) -> &str {
        self.content.split_once("\r\n").unwrap().1
    }
    pub fn get_headers(&self) -> Split<'_, &str> {
        self.get_headers_raw().split("\r\n")
    }
    pub fn get_headers_parsed(
        &'a self,
    ) -> Map<Split<'a, &'a str>, impl FnMut(&'a str) -> HTTPHeaderView<'a>> {
        self.get_headers().map(|header| {
            let (key, value) = header.split_once(':').unwrap();
            HTTPHeaderView { key, value }
        })
    }
    pub fn get_method(&self) -> HTTPMethod {
        let method_name = self.get_first_line().split_once(' ').unwrap().0;
        if method_name.eq_ignore_ascii_case("GET") {
            HTTPMethod::Get
        } else {
            todo!("{}", self.content)
        }
    }
    pub fn get_version(&self) -> HTTPVersion {
        let version = self.get_first_line().rsplit_once(' ').unwrap().1;
        assert_eq!(&version[0..5], "HTTP/");

        let (major, minor) = version[5..]
            .split_once('.')
            .map_or((0, 0), |(major, minor)| {
                (major.parse().unwrap(), minor.parse().unwrap())
            });
        HTTPVersion { major, minor }
    }
    pub fn get_path(&self) -> &str {
        self.get_first_line()
            .split_once(' ')
            .unwrap()
            .1
            .rsplit_once(' ')
            .unwrap()
            .0
    }
}

impl Debug for HTTPRequestHeaderView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HTTPHeaderView")
            .field("method", &self.get_method())
            .field("path", &self.get_path())
            .field("version", &self.get_version())
            .field("headers", &self.get_headers_raw())
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HTTPRequestHeader {
    method: HTTPMethod,
    path: String,
    version: HTTPVersion,
    headers: HashMap<String, String>,
}

impl Prepare for HTTPRequestHeader {}
impl Data for HTTPRequestHeader {
    fn size(&self) -> usize {
        todo!()
        // self.method.as_str().len()
    }
}

impl WriteTo for HTTPRequestHeader {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(self.method.as_str().as_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(b"HTTP/")?;
        writer.write_all(&[self.version.major + b'0'])?;
        writer.write_all(b".")?;
        writer.write_all(&[self.version.minor + b'0'])?;
        writer.write_all(b"\r\n")?;
        todo!();
    }
}

pub type HTTPRequestPacket<C = Vec<u8>> = Packet<HTTPRequestHeader, C>;
pub type HTTPRequestPacketView<'a, C = &'a [u8]> = PacketView<'a, HTTPRequestHeaderView<'a>, C>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HTTPResponseHeader {
    pub version: HTTPVersion,
    pub code: u16,
    pub reason: String,
    pub headers: HashMap<String, String>,
}

impl Prepare for HTTPResponseHeader {
    fn prepare(&mut self) {
        if self.reason.is_empty() {
            match self.code {
                200 => self.reason.push_str("OK"),
                500 => self.reason.push_str("SERVER ERROR"),
                _ => {}
            }
        }
    }
}
impl Data for HTTPResponseHeader {
    fn size(&self) -> usize {
        14 + self.code.to_string().len()
            + self.reason.len()
            + self
                .headers
                .iter()
                .map(|(key, value)| key.len() + value.len() + 4)
                .sum::<usize>()
    }
}

impl WriteTo for HTTPResponseHeader {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(b"HTTP/")?;
        writer.write_all(&[self.version.major + b'0'])?;
        writer.write_all(b".")?;
        writer.write_all(&[self.version.minor + b'0'])?;
        writer.write_all(b" ")?;
        writer.write_all(self.code.to_string().as_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(self.reason.as_bytes())?;
        writer.write_all(b"\r\n")?;
        for header in &self.headers {
            writer.write_all(header.0.as_bytes())?;
            writer.write_all(b": ")?;
            writer.write_all(header.1.as_bytes())?;
            writer.write_all(b"\r\n")?;
        }
        writer.write_all(b"\r\n")?;
        Ok(self.size())
    }
}

pub type HTTPResponsePacket<C = Vec<u8>> = Packet<HTTPResponseHeader, C>;
// pub type HTTPResponsePacketView<'a, C = &'a [u8]> = PacketView<'a, HTTPResponseHeaderView<'a>, C>;
