#![allow(unused)]

use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    io,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IpProtocol {
    Icmp = 1,
    Igmp = 2,
    Tcp = 6,
    Udp = 17,
    IpV6Encap = 41,
    Ospf = 89,
    Sctp = 132,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IPV4Header {
    version_and_ihl: u8,
    dscp_and_ecn: u8,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: u16,
    ttl: u8,
    protocol: IpProtocol,
    header_checksum: u16,
    source_address: IpV4Addr,
    destination_address: IpV4Addr,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IPV4Packet<'a> {
    header: IPV4Header,
    data: Cow<'a, [u8]>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IpV4Addr(u32);

impl From<u32> for IpV4Addr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl IPV4Header {
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice[0] & 0xF != 5 {
            unimplemented!("Options in header is not implemented")
        }
        let total_length = u16::from_be_bytes([slice[2], slice[3]]);
        Self {
            version_and_ihl: slice[0],
            dscp_and_ecn: slice[1],
            total_length,
            identification: u16::from_be_bytes([slice[4], slice[5]]),
            flags_and_fragment_offset: ((slice[6] as u16) << 8) + slice[7] as u16,
            ttl: slice[8],
            protocol: unsafe { core::mem::transmute::<u8, IpProtocol>(slice[9]) },
            header_checksum: u16::from_be_bytes([slice[10], slice[11]]),
            source_address: u32::from_be_bytes([slice[12], slice[13], slice[14], slice[15]]).into(),
            destination_address: u32::from_be_bytes([slice[16], slice[17], slice[18], slice[19]])
                .into(),
        }
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.version_and_ihl);
        bytes.push(self.dscp_and_ecn);
        bytes.extend_from_slice(&u16::to_be_bytes(self.total_length));
        bytes.extend_from_slice(&u16::to_be_bytes(self.identification));
        bytes.push((self.flags_and_fragment_offset >> 8) as u8);
        bytes.push((self.flags_and_fragment_offset & 0xFF) as u8);
        bytes.push(self.ttl);
        bytes.push(self.protocol as u8);
        bytes.extend_from_slice(&u16::to_be_bytes(self.header_checksum));
        bytes.extend_from_slice(&u32::to_be_bytes(self.source_address.0));
        bytes.extend_from_slice(&u32::to_be_bytes(self.destination_address.0));
        bytes
    }
    pub fn get_version(&self) -> u8 {
        self.version_and_ihl >> 4
    }
    pub fn get_ihl(&self) -> u8 {
        self.version_and_ihl & 0xF
    }
    pub fn get_dscp(&self) -> u8 {
        self.version_and_ihl >> 2
    }
    pub fn get_ecn(&self) -> u8 {
        self.version_and_ihl & 0b111
    }
    pub fn get_total_length(&self) -> u16 {
        self.total_length
    }
    pub fn get_identification(&self) -> u16 {
        self.identification
    }
    pub fn get_flags(&self) -> u8 {
        (self.flags_and_fragment_offset >> 13) as u8
    }
    pub fn get_fragment_offset(&self) -> u16 {
        u16::from_be(self.flags_and_fragment_offset & 0x1FFF)
    }
    pub fn get_ttl(&self) -> u8 {
        self.ttl
    }
    pub fn get_protocol(&self) -> IpProtocol {
        self.protocol
    }
    pub fn get_header_checksum(&self) -> u16 {
        self.header_checksum
    }
    pub fn get_source_address(&self) -> IpV4Addr {
        self.source_address
    }
    pub fn get_destination_address(&self) -> IpV4Addr {
        self.destination_address
    }
}

impl<'a> IPV4Packet<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        let header = IPV4Header::from_slice(slice);
        Self {
            data: slice[20..header.total_length as usize].into(),
            header,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        bytes.extend_from_slice(&self.data);
        bytes
    }

    pub fn get_header(&self) -> &IPV4Header {
        &self.header
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
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

impl Debug for IPV4Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IPV4Header")
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
impl Debug for IPV4Packet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = format!("{:?}", self.get_data());
        f.debug_struct("IPV4Packet")
            .field("header", &self.get_header())
            .field("data", &data)
            .finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ICMPPacket {
    message_type: u8,
    code: u8,
    checksum: u16,
}

impl ICMPPacket {
    pub fn get_message_type(&self) -> u8 {
        self.message_type
    }
    pub fn get_code(&self) -> u8 {
        self.code
    }
    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }
}

impl Debug for ICMPPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ICMPHeader")
            .field("message_type", &self.get_message_type())
            .field("code", &self.get_code())
            .field("checksum", &self.get_checksum())
            .finish()
    }
}

impl ICMPPacket {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            message_type: slice[0],
            code: slice[1],
            checksum: u16::from_be_bytes([slice[2], slice[3]]),
        }
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.message_type);
        bytes.push(self.code);
        bytes.extend_from_slice(&u16::to_be_bytes(self.checksum));
        bytes
    }
    pub fn compute_checksum(&mut self) {
        self.checksum = 0;
    }
}

fn main() -> io::Result<()> {
    // Create a new TUN interface named "tun0" in TUN mode.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
    let mut buf = [0u8; 1504];

    // Main loop to continuously receive data from the interface.
    loop {
        // Receive data from the TUN interface and store the number of bytes received in `nbytes`.
        let nbytes = nic.recv(&mut buf[..])?;

        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        if proto != 0x0800 {
            // Not an IP packet
            continue;
        }

        let mut ip_packet = IPV4Packet::from_slice(&buf[4..]);
        let icmp_packet = ICMPPacket::from_slice(ip_packet.get_data());

        let mut icmp_response = icmp_packet;

        let mut ip_response = ip_packet.clone();
        (
            ip_response.header.destination_address,
            ip_response.header.source_address,
        ) = (
            ip_response.header.source_address,
            ip_response.header.destination_address,
        );

        icmp_response.message_type = 0;

        ip_response.data = icmp_response.to_bytes().into();

        nic.send(&ip_response.to_bytes());

        println!("read packet: {:#?}", icmp_packet);
    }
}
