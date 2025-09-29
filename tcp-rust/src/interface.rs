use crate::{
    ip::{IPV4PacketView, IpProtocol},
    traits::{PayloadView, WriteTo},
    tun_tap,
};

#[derive(Debug)]
pub struct Interface {
    interface: tun_tap::Interface,
    // Define a buffer of size 1504 bytes (maximum Ethernet frame size without CRC) to store received data.
    buffer: [u8; 1504],
    pub nbytes: usize,
}

impl Interface {
    pub fn new(ifname: &str) -> Self {
        // Create a new TUN interface named "tun0" in TUN mode.
        let nic = tun_tap::Interface::new(ifname, tun_tap::Mode::Tun).unwrap();

        Self {
            interface: nic,
            buffer: [0; _],
            nbytes: 0,
        }
    }

    pub fn receive(&mut self) {
        // Receive data from the TUN interface and store the number of bytes received in `nbytes`.
        self.nbytes = self.interface.recv(&mut self.buffer[..]).unwrap();
    }
    pub fn send(&mut self) {
        self.interface
            .send(&mut self.buffer[..self.nbytes])
            .unwrap();
    }
    pub fn write(&mut self, writter: impl WriteTo) {
        self.nbytes = writter.write_into(&mut &mut self.buffer[4..]).unwrap() + 4;
    }

    pub fn get_proto(&self) -> u16 {
        u16::from_be_bytes([self.buffer[2], self.buffer[3]])
    }
    pub fn get_flags(&self) -> u16 {
        u16::from_be_bytes([self.buffer[0], self.buffer[1]])
    }

    pub fn get_packet<'a, T: PayloadView<'a>>(&'a self) -> T {
        T::from_slice(&self.buffer[4..self.nbytes])
    }

    pub fn get_ip_protocol(&self) -> IpProtocol {
        self.get_packet::<IPV4PacketView>()
            .get_header()
            .get_protocol()
    }
}
