#![allow(incomplete_features)]
#![feature(specialization, slice_split_once)]

pub mod checksum;
pub mod http;
pub mod icmp;
pub mod interface;
pub mod ip;
pub mod packet;
pub mod tcp;
pub mod traits;
pub mod tun_tap;

use std::io;

use crate::{
    icmp::ICMPPacketView,
    interface::Interface,
    ip::{IPV4HeaderView, IPV4PacketView, IpProtocol},
    tcp::manager::TCPManager,
    traits::ToMutable,
};

fn main() -> io::Result<()> {
    let mut interface =
        Interface::new(tun_tap::Interface::new("tun%d", tun_tap::Mode::Tun).unwrap());
    let mut tcp_manager = TCPManager::new();

    loop {
        interface.receive();

        if !interface.is_ip() {
            // Not an IP packet
            println!("Not an IP Packet: {}", interface.get_proto());
            continue;
        }

        if interface.get_ip_protocol() == IpProtocol::Icmp {
            let ip_packet = interface.get_packet::<IPV4PacketView<ICMPPacketView>>();
            let mut ip_response = ip_packet.to_mutable();

            ip_response.header.answer();

            ip_response.payload.header.message_type = 0;
            ip_response.payload.header.code = 0;

            interface.write(ip_response);

            interface.send();
            println!("answered an echo packet");
        } else if interface.get_ip_protocol() == IpProtocol::Tcp {
            tcp_manager.handle_tcp_packet(&mut interface);
        } else {
            println!(
                "received a non ICMP packet, protocol {:?}",
                interface.get_packet::<IPV4HeaderView>().get_protocol()
            );
        }
    }
}
