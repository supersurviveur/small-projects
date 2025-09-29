use std::{
    fmt::Debug,
    io::{self, Write},
};

use crate::{
    ip::IPV4Packet,
    packet::{Packet, PacketView},
    traits::{AsArrayUnchecked, Header, HeaderView, Payload, Prepare, ToMutable, WriteTo},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TCPHeaderView<'a> {
    content: &'a [u8],
}

impl ToMutable for TCPHeaderView<'_> {
    type MutableType = TCPHeader;

    fn to_mutable(&self) -> Self::MutableType {
        TCPHeader {
            source_port: self.get_source_port(),
            destination_port: self.get_destination_port(),
            sequence_number: self.get_sequence_number(),
            acknowledgement_number: self.get_acknowledgement_number(),
            data_offset: self.get_data_offset(),
            reserved: self.get_reserved(),
            cwr: self.get_cwr(),
            ece: self.get_ece(),
            urg: self.get_urg(),
            ack: self.get_ack(),
            psh: self.get_psh(),
            rst: self.get_rst(),
            syn: self.get_syn(),
            fin: self.get_fin(),
            window: self.get_window(),
            checksum: self.get_checksum(),
            urgent_pointer: self.get_urgent_pointer(),
            options: self.get_parsed_options(),
        }
    }
}

impl<'a> HeaderView<'a> for TCPHeaderView<'a> {
    fn from_slice(slice: &'a [u8]) -> Self {
        let length = (slice[12] >> 4) as usize * 4;
        Self {
            content: &slice[..length],
        }
    }

    fn size(&self) -> usize {
        self.content.len()
    }
    fn as_bytes(&self) -> &'a [u8] {
        self.content
    }
}

impl<'a> TCPHeaderView<'a> {
    pub fn get_source_port(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[0..2].as_array_unchecked() })
    }
    pub fn get_destination_port(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[2..4].as_array_unchecked() })
    }
    pub fn get_sequence_number(&self) -> u32 {
        u32::from_be_bytes(*unsafe { self.content[4..8].as_array_unchecked() })
    }
    pub fn get_acknowledgement_number(&self) -> u32 {
        u32::from_be_bytes(*unsafe { self.content[8..12].as_array_unchecked() })
    }
    pub fn get_data_offset(&self) -> u8 {
        self.content[12] >> 4
    }
    pub fn get_reserved(&self) -> u8 {
        self.content[12] & 0xF
    }
    pub fn get_cwr(&self) -> bool {
        (self.content[13] & (1 << 7)) != 0
    }
    pub fn get_ece(&self) -> bool {
        (self.content[13] & (1 << 6)) != 0
    }
    pub fn get_urg(&self) -> bool {
        (self.content[13] & (1 << 5)) != 0
    }
    pub fn get_ack(&self) -> bool {
        (self.content[13] & (1 << 4)) != 0
    }
    pub fn get_psh(&self) -> bool {
        (self.content[13] & (1 << 3)) != 0
    }
    pub fn get_rst(&self) -> bool {
        (self.content[13] & (1 << 2)) != 0
    }
    pub fn get_syn(&self) -> bool {
        (self.content[13] & (1 << 1)) != 0
    }
    pub fn get_fin(&self) -> bool {
        (self.content[13] & (1 << 0)) != 0
    }
    pub fn get_window(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[14..16].as_array_unchecked() })
    }
    pub fn get_checksum(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[16..18].as_array_unchecked() })
    }
    pub fn get_urgent_pointer(&self) -> u16 {
        u16::from_be_bytes(*unsafe { self.content[18..20].as_array_unchecked() })
    }
    pub fn get_options(&self) -> &[u8] {
        &self.content[20..self.size() - 20]
    }
    pub fn get_parsed_options(&self) -> Vec<TCPOption> {
        let mut i = 20;
        let mut res = vec![];
        while i < self.size() {
            let kind = self.content[i];
            let len = if kind >= 2 { self.content[i + 1] } else { 1 };
            res.push(TCPOption {
                kind,
                len,
                data: if len > 2 {
                    Some(self.content[i + 2..i + len as usize].to_vec())
                } else {
                    None
                },
            });
            i += len as usize;
        }
        res
    }
}

impl Debug for TCPHeaderView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TCPHeaderView")
            .field("source_port", &self.get_source_port())
            .field("destination_port", &self.get_destination_port())
            .field("sequence_number", &self.get_sequence_number())
            .field("acknowledgement_number", &self.get_acknowledgement_number())
            .field("data_offset", &self.get_data_offset())
            .field("reserved", &self.get_reserved())
            .field("cwr", &self.get_cwr())
            .field("ece", &self.get_ece())
            .field("urg", &self.get_urg())
            .field("ack", &self.get_ack())
            .field("psh", &self.get_psh())
            .field("rst", &self.get_rst())
            .field("syn", &self.get_syn())
            .field("fin", &self.get_fin())
            .field("window", &self.get_window())
            .field("checksum", &self.get_checksum())
            .field("urgent_pointer", &self.get_urgent_pointer())
            .field("options", &self.get_parsed_options())
            .finish()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TCPOption {
    kind: u8,
    len: u8,
    data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TCPHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub acknowledgement_number: u32,
    pub data_offset: u8,
    pub reserved: u8,
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Vec<TCPOption>,
}

impl Prepare for TCPHeader {
    fn prepare(&mut self) {
        self.set_size();
    }
}
impl<'a> Header<'a> for TCPHeader {
    type ViewType<'b> = TCPHeaderView<'b>;

    fn size(&self) -> usize {
        self.data_offset as usize * 4
    }
}
impl WriteTo for TCPHeader {
    fn write_to_inner<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&u16::to_be_bytes(self.source_port))?;
        writer.write_all(&u16::to_be_bytes(self.destination_port))?;
        writer.write_all(&u32::to_be_bytes(self.sequence_number))?;
        writer.write_all(&u32::to_be_bytes(self.acknowledgement_number))?;
        writer.write_all(&[self.data_offset << 4 | self.reserved])?;
        writer.write_all(&[(self.cwr as u8) << 7
            | (self.ece as u8) << 6
            | (self.urg as u8) << 5
            | (self.ack as u8) << 4
            | (self.psh as u8) << 3
            | (self.rst as u8) << 2
            | (self.syn as u8) << 1
            | (self.fin as u8)])?;

        writer.write_all(&u16::to_be_bytes(self.window))?;
        writer.write_all(&u16::to_be_bytes(self.checksum))?;
        writer.write_all(&u16::to_be_bytes(self.urgent_pointer))?;
        let mut current_size = 0;
        for option in &self.options {
            writer.write_all(&[option.kind])?;
            if option.len >= 2 {
                writer.write_all(&[option.len])?;
                if let Some(data) = &option.data {
                    writer.write_all(data)?;
                }
            }
            current_size += option.len;
        }
        if current_size % 4 != 0 {
            for _ in 0..4 - current_size % 4 {
                writer.write_all(&[1])?;
            }
        }
        Ok(self.size())
    }
}
impl TCPHeader {
    pub fn set_size(&mut self) {
        let current_size: u16 = self.options.iter().map(|option| option.len as u16).sum();
        self.data_offset = 5 + current_size.div_ceil(4) as u8;
    }

    pub fn answer(&mut self, incoming_packet: TCPHeaderView) {
        (self.destination_port, self.source_port) = (
            incoming_packet.get_source_port(),
            incoming_packet.get_destination_port(),
        );
    }
}

pub type TCPPacket<'a, C = Vec<u8>> = Packet<'a, TCPHeader, C>;
pub type TCPPacketView<'a, C = &'a [u8]> = PacketView<'a, TCPHeaderView<'a>, C>;

impl<'a, C: Payload<'a>> Prepare for IPV4Packet<'a, TCPPacket<'a, C>> {
    fn prepare(&mut self) {
        self.header.prepare();
        self.payload.prepare();
        self.header.prepare_ip_header(self.size());
        self.payload.header.checksum = 0;
        let checksum = self
            .payload
            .compute_checksum()
            .add_4bytes(self.header.source_address.0.to_be_bytes())
            .add_4bytes(self.header.destination_address.0.to_be_bytes())
            .add_byte(self.header.protocol as u8)
            .add_2bytes((self.payload.payload.size() as u16).to_be_bytes())
            .add_2bytes((self.payload.header.data_offset as u16 * 4).to_be_bytes());
        self.payload.header.checksum = checksum.ones_complement()
    }
}
