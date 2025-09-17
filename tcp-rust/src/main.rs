#![allow(unused)]

pub mod icmp;
pub mod ip;
pub mod tcp;
pub mod traits;
pub mod tun_tap;

use std::{
    any::{Any, TypeId},
    borrow::Cow,
    clone,
    fmt::{Debug, Display},
    io::{self, Write},
    marker::PhantomData,
};

use crate::{
    icmp::ICMPHeaderView,
    ip::{IPV4HeaderView, IpProtocol},
    tcp::TCPHeaderView,
    traits::{Header, HeaderView, Payload, PayloadView, ToMutable, WriteTo},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PacketView<'a, H: HeaderView<'a>, C: PayloadView<'a>> {
    header: H,
    payload: C,
    phantom: PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Packet<'a, H: Header<'a>, C: Payload<'a>> {
    header: H,
    payload: C,
    phantom: PhantomData<&'a ()>,
}

impl<'a, H: Header<'a>, C: Payload<'a>> WriteTo for Packet<'a, H, C> {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.header.write_to(writer)?;
        self.payload.write_to(writer)?;
        Ok(())
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
}

impl<'a> Payload<'a> for Vec<u8> {
    type ViewType = &'a [u8];

    fn size(&self) -> usize {
        self.len()
    }
}

trait AsArrayUnchecked<T> {
    unsafe fn as_array_unchecked<const N: usize>(&self) -> &[T; N];
}

impl<T> AsArrayUnchecked<T> for [T] {
    unsafe fn as_array_unchecked<const N: usize>(&self) -> &[T; N] {
        unsafe { &*(self.as_ptr() as *const _) }
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

impl<'a, H: HeaderView<'a>, C: PayloadView<'a>> PayloadView<'a> for PacketView<'a, H, C> {
    fn from_slice(slice: &'a [u8]) -> Self {
        Self::from_slice(slice)
    }
}
impl<'a, H: Header<'a>, C: Payload<'a>> Payload<'a> for Packet<'a, H, C> {
    type ViewType = PacketView<'a, H::ViewType, C::ViewType>;

    fn size(&self) -> usize {
        self.header.size() + self.payload.size()
    }
}

impl<'a, H: Header<'a>, C: Payload<'a>> Packet<'a, H, C> {
    pub fn compute_checksum(&self) -> u16 {
        let (sum, carry) = self
            .header
            .compute_checksum()
            .overflowing_add(self.payload.compute_checksum());
        sum + carry as u16
    }
}

fn main() -> io::Result<()> {
    // Create a new TUN interface named "tun0" in TUN mode.
    let nic = tun_tap::Interface::new("tun%d", tun_tap::Mode::Tun)?;

    // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
    let mut buf = [0u8; 1504];
    let mut response_buf = [0u8; 1504];

    // Main loop to continuously receive data from the interface.
    loop {
        // Receive data from the TUN interface and store the number of bytes received in `nbytes`.
        let nbytes = nic.recv(&mut buf[..])?;

        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        if proto != 0x0800 {
            // Not an IP packet
            println!("Not an IP Packet: {}", proto);
            continue;
        }

        if IPV4HeaderView::from_slice(&buf[4..nbytes]).get_protocol() == IpProtocol::Icmp {
            let mut ip_packet = PacketView::<
                '_,
                IPV4HeaderView,
                PacketView<'_, ICMPHeaderView, &[u8]>,
            >::from_slice(&buf[4..nbytes]);
            // println!("read packet: {:#?}", ip_packet);

            let mut ip_response = ip_packet.to_mutable();
            (
                ip_response.header.destination_address,
                ip_response.header.source_address,
            ) = (
                ip_response.header.source_address,
                ip_response.header.destination_address,
            );

            ip_response.payload.header.message_type = 0;
            ip_response.payload.header.code = 0;
            ip_response.payload.header.checksum = 0;
            ip_response.payload.header.checksum = !ip_response.payload.compute_checksum();
            ip_response.header.set_checksum();

            let nbytes = ip_response.size();
            ip_response.write_into(&mut &mut response_buf[4..])?;

            response_buf[0..4].copy_from_slice(&buf[0..4]);
            nic.send(&response_buf[..nbytes + 4]);
            println!("answered an echo packet");
        } else if IPV4HeaderView::from_slice(&buf[4..nbytes]).get_protocol() == IpProtocol::Tcp {
            let mut ip_packet =
                PacketView::<'_, IPV4HeaderView, PacketView<'_, TCPHeaderView, &[u8]>>::from_slice(
                    &buf[4..nbytes],
                );

            let mut ip_response = ip_packet.to_mutable();

            (
                ip_response.header.destination_address,
                ip_response.header.source_address,
            ) = (
                ip_response.header.source_address,
                ip_response.header.destination_address,
            );

            (
                ip_response.payload.header.destination_port,
                ip_response.payload.header.source_port,
            ) = (
                ip_response.payload.header.source_port,
                ip_response.payload.header.destination_port,
            );
            ip_response.payload.header.ack = true;
            ip_response.payload.header.syn = true;
            ip_response.payload.header.acknowledgement_number =
                ip_response.payload.header.sequence_number + 1;
            ip_response.payload.header.sequence_number = 3453253245;
            // ip_response.payload.header.options.remove(0);
            // ip_response.payload.header.options.remove(0);
            ip_response.payload.header.set_size();

            ip_response.payload.header.checksum = 0;
            let mut checksum = ip_response.payload.compute_checksum();
            let mut carry = false;
            (checksum, carry) =
                checksum.overflowing_add((ip_response.header.source_address.0 >> 16) as u16);
            (checksum, carry) =
                checksum.carrying_add(ip_response.header.source_address.0 as u16, carry);
            (checksum, carry) =
                checksum.overflowing_add((ip_response.header.destination_address.0 >> 16) as u16);
            (checksum, carry) =
                checksum.carrying_add(ip_response.header.destination_address.0 as u16, carry);
            (checksum, carry) = checksum.carrying_add(ip_response.header.protocol as u16, carry);
            (checksum, carry) =
                checksum.carrying_add(ip_response.payload.header.data_offset as u16, carry);
            (checksum, carry) =
                checksum.carrying_add((ip_response.payload.payload.size() / 4) as u16, carry);
            checksum += carry as u16;
            ip_response.payload.header.checksum = !checksum + 0x1e4;

            ip_response.header.total_length =
                (ip_response.header.size() + ip_response.payload.size()) as u16;
            ip_response.header.set_checksum();
            let nbytes = ip_response.size();
            ip_response.write_into(&mut &mut response_buf[4..])?;
            response_buf[0..4].copy_from_slice(&buf[0..4]);
            nic.send(&response_buf[..nbytes + 4]);

            // println!("TCP packet: {:#?}", ip_packet);
            println!("answered TCP packet");
        } else {
            println!(
                "received a non ICMP packet, protocol {:?}",
                IPV4HeaderView::from_slice(&buf[4..nbytes]).get_protocol()
            );
        }
    }
}
