use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::{
    ip::{IPV4PacketView, IpProtocol},
    traits::WriteTo,
};

#[derive(Debug)]
pub struct Interface<T: Read + Write> {
    interface: T,
    // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
    buffer: [u8; 1504],
    pub nbytes: usize,
}

impl<T: Read + Write> Interface<T> {
    pub fn new(interface: T) -> Self {
        Self {
            interface,
            buffer: [0; _],
            nbytes: 0,
        }
    }

    pub fn receive(&mut self) {
        // Receive data from the TUN interface and store the number of bytes received in `nbytes`.
        self.nbytes = self.interface.read(&mut self.buffer[..]).unwrap();
    }
    pub fn send(&mut self) {
        self.interface
            .write_all(&self.buffer[..self.nbytes])
            .unwrap();
    }
    pub fn write(&mut self, mut writter: impl WriteTo) {
        self.nbytes = writter.write_to(&mut &mut self.buffer[4..]).unwrap() + 4;
    }

    pub fn get_proto(&self) -> u16 {
        u16::from_be_bytes([self.buffer[2], self.buffer[3]])
    }
    pub fn get_flags(&self) -> u16 {
        u16::from_be_bytes([self.buffer[0], self.buffer[1]])
    }

    pub fn get_packet<'a, V: TryFrom<&'a [u8]>>(&'a self) -> V
    where
        <V as TryFrom<&'a [u8]>>::Error: Debug,
    {
        V::try_from(&self.buffer[4..self.nbytes]).unwrap()
    }

    pub fn try_get_packet<'a, V: TryFrom<&'a [u8]>>(
        &'a self,
    ) -> Result<V, <V as TryFrom<&'a [u8]>>::Error> {
        V::try_from(&self.buffer[4..self.nbytes])
    }

    pub fn is_ip(&self) -> bool {
        self.get_proto() == 0x0800
    }

    pub fn get_ip_protocol(&self) -> IpProtocol {
        self.get_packet::<IPV4PacketView>()
            .get_header()
            .get_protocol()
    }
}
