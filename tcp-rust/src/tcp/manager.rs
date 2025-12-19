use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{
    http::{HTTPRequestHeaderView, HTTPResponseHeader, HTTPResponsePacket},
    interface::Interface,
    ip::{IPV4Packet, IPV4PacketView},
    tcp::{TCPHeader, TCPPacket, TCPPacketView},
    traits::{Data, ToMutable},
};

#[derive(Debug, Clone, Default)]
pub struct TCPManager {
    connections: HashMap<u16, TCPConnection>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TCPConnectionState {
    #[default]
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
    Closed,
}

#[derive(Debug, Clone, Default)]
pub struct TCPConnection {
    state: TCPConnectionState,
    sequence_number: u32,
}

impl TCPManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn handle_tcp_packet(&mut self, interface: &mut Interface<impl Read + Write>) {
        let ip_packet = interface.get_packet::<IPV4PacketView<TCPPacketView>>();

        if ip_packet.payload.header.get_syn()
            && !ip_packet.payload.header.get_ack()
            && !self
                .connections
                .contains_key(&ip_packet.payload.header.get_destination_port())
        {
            self.connections.insert(
                ip_packet.payload.header.get_destination_port(),
                TCPConnection::new(),
            );
        }
        let destination_port = ip_packet.payload.header.get_destination_port();
        self.connections
            .entry(destination_port)
            .and_modify(|connection| connection.handle_packet(interface));

        if self
            .connections
            .get(&destination_port)
            .is_some_and(|conn| conn.state == TCPConnectionState::Closed)
        {
            self.connections.remove(&destination_port);
        }
    }
}

impl TCPConnection {
    pub fn new() -> Self {
        Self {
            state: TCPConnectionState::Listen,
            sequence_number: 0,
        }
    }
    pub fn handle_packet(&mut self, interface: &mut Interface<impl Read + Write>) {
        let ip_packet = interface.get_packet::<IPV4PacketView<TCPPacketView>>();
        let tcp_packet = ip_packet.payload;
        match self.state {
            TCPConnectionState::Listen
                if tcp_packet.header.get_syn() && !tcp_packet.header.get_ack() =>
            {
                self.sequence_number = 3453253245;

                let mut response = self.make_ack_packet(ip_packet);
                response.payload.header.syn = true;

                self.sequence_number += 1;

                interface.write(response);
                interface.send();

                self.state = TCPConnectionState::SynReceived;
            }
            TCPConnectionState::SynReceived
                if !tcp_packet.header.get_syn() && tcp_packet.header.get_ack() =>
            {
                self.state = TCPConnectionState::Established;
            }
            TCPConnectionState::Established if tcp_packet.header.get_fin() => {
                let ack_response = self.make_ack_packet(ip_packet);

                let mut ip_response_fin = IPV4Packet::new(
                    ack_response.header,
                    TCPPacket::new(TCPHeader::default(), vec![]),
                );

                let tcp_response_fin = &mut ip_response_fin.payload.header;
                tcp_response_fin.answer(tcp_packet.header);
                tcp_response_fin.fin = true;
                tcp_response_fin.sequence_number = self.sequence_number;
                tcp_response_fin.acknowledgement_number = tcp_packet.header.get_sequence_number()
                    + tcp_packet.get_payload().size() as u32;
                tcp_response_fin.window = tcp_packet.header.get_window();

                interface.write(ack_response);
                interface.send();

                self.state = TCPConnectionState::CloseWait;

                interface.write(ip_response_fin);
                interface.send();

                self.state = TCPConnectionState::LastAck;
            }
            TCPConnectionState::LastAck => {
                if tcp_packet.header.get_ack() && tcp_packet.payload.is_empty() {
                    self.state = TCPConnectionState::Closed;
                }
            }
            TCPConnectionState::Established => {
                if tcp_packet.header.get_ack() && tcp_packet.payload.is_empty() {
                    return;
                }
                let http_packet = HTTPRequestHeaderView::try_from(tcp_packet.payload);
                let http_packet = match http_packet {
                    Ok(p) => p,
                    // Not an http packet, exit
                    Err(_) => return,
                };

                let ack_response = self.make_ack_packet(ip_packet);

                let mut http_response = IPV4Packet::new(
                    ack_response.header,
                    TCPPacket::new(
                        TCPHeader::default(),
                        HTTPResponsePacket::new(
                            HTTPResponseHeader::default(),
                            "<html><b>Hello</b> World !</html>",
                        ),
                    ),
                );

                let tcp_response = &mut http_response.payload.header;
                tcp_response.answer(tcp_packet.header);
                tcp_response.sequence_number = self.sequence_number;
                self.sequence_number += http_response.payload.payload.payload.size() as u32;
                tcp_response.window = tcp_packet.header.get_window();
                tcp_response.ack = true;
                tcp_response.acknowledgement_number =
                    ack_response.payload.header.acknowledgement_number;

                let http_header = &mut http_response.payload.payload.header;
                http_header.version = http_packet.get_version();
                http_header.code = 200;
                http_header.headers.extend([
                    (
                        "Content-Type".to_string(),
                        "text/html; charset=UTF-8".to_string(),
                    ),
                    (
                        "Content-Length".to_string(),
                        http_response.payload.payload.payload.len().to_string(),
                    ),
                    ("Connection".to_string(), "close".to_string()),
                ]);

                interface.write(ack_response);
                interface.send();

                interface.write(http_response);
                interface.send();
            }
            _ => {}
        }

        println!("answered TCP packet");
    }

    fn make_ack_packet<'a>(
        &self,
        ip_packet: IPV4PacketView<'a, TCPPacketView<'a>>,
    ) -> IPV4Packet<TCPPacket> {
        let tcp_packet = ip_packet.payload;

        let mut ip_response = IPV4Packet::new(
            ip_packet.header.to_mutable(),
            TCPPacket::new(TCPHeader::default(), vec![]),
        );

        ip_response.header.answer();

        let tcp_response = &mut ip_response.payload.header;
        tcp_response.answer(tcp_packet.header);
        tcp_response.ack = true;
        tcp_response.sequence_number = self.sequence_number;
        tcp_response.acknowledgement_number = tcp_packet.header.get_sequence_number() + 1;
        tcp_response.window = tcp_packet.header.get_window();

        ip_response
    }
}
