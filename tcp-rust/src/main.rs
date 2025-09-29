#![allow(incomplete_features)]
#![feature(specialization)]

pub mod checksum;
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
    ip::{IPV4HeaderView, IPV4Packet, IPV4PacketView, IpProtocol},
    tcp::{TCPHeader, TCPPacket, TCPPacketView},
    traits::{HeaderView, Payload, PayloadView, ToMutable, WriteTo},
};

fn main() -> io::Result<()> {
    let mut interface = Interface::new("tun%d");

    loop {
        interface.receive();

        let _flags = interface.get_flags();
        let proto = interface.get_proto();

        if proto != 0x0800 {
            // Not an IP packet
            println!("Not an IP Packet: {}", proto);
            continue;
        }

        if interface.get_ip_protocol() == IpProtocol::Icmp {
            let mut ip_response;
            {
                let ip_packet = interface.get_packet::<IPV4PacketView<ICMPPacketView>>();
                ip_response = ip_packet.to_mutable();
            }

            ip_response.header.answer();

            ip_response.payload.header.message_type = 0;
            ip_response.payload.header.code = 0;

            interface.write(ip_response);

            interface.send();
            println!("answered an echo packet");
        } else if interface.get_ip_protocol() == IpProtocol::Tcp {
            let ip_packet = interface.get_packet::<IPV4PacketView<TCPPacketView>>();

            let mut ip_response = IPV4Packet::new(
                ip_packet.header.to_mutable(),
                TCPPacket::new(TCPHeader::default(), vec![]),
            );

            ip_response.header.answer();

            let tcp_response = &mut ip_response.payload.header;
            tcp_response.answer(ip_packet.payload.header);
            tcp_response.ack = true;
            tcp_response.syn = true;
            tcp_response.acknowledgement_number =
                ip_packet.payload.header.get_sequence_number() + 1;
            tcp_response.sequence_number = 3453253245;
            // tcp_response.options = ip_packet.payload.header.get_parsed_options();
            tcp_response.window = ip_packet.payload.header.get_window();

            interface.write(ip_response);
            interface.send();

            println!("answered TCP packet");
        } else {
            // println!(
            //     "received a non ICMP packet, protocol {:?}",
            //     interface.get_packet::<IPV4HeaderView>().get_protocol()
            // );
        }
    }
}
