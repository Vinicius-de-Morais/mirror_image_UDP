use std::io::{self, Write};
use std::{fmt, fs::File, net::UdpSocket};

pub mod protocol;


pub fn handle_request(socket: UdpSocket, client_addr: std::net::SocketAddr, request_packet: &[u8]) -> io::Result<()> {    
    // Protocolo para lidar com
    let mut protocol = protocol::Protocol::new();
    protocol.handle_request(socket, client_addr, request_packet)?;

    println!("File received successfully!");
    Ok(())
}


#[derive(Debug, PartialEq)]
pub struct Package {
    pub package_type: PackageType,
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl Package {

    pub fn new_syn() -> Package {
        Package {
            package_type: PackageType::SYN,
            sequence: 0,
            data: vec![],
        }
    }

    pub fn new_ack() -> Package {
        Package {
            package_type: PackageType::ACK,
            sequence: 0,
            data: vec![],
        }
    }

    pub fn new_nak() -> Package {
        Package {
            package_type: PackageType::NAK,
            sequence: 0,
            data: vec![],
        }
    }

    pub fn new_data(sequence: u8, data: &[u8]) -> Package {
        Package {
            package_type: PackageType::PKG,
            sequence,
            data: data.to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Package> {
        if bytes.len() < 4 {
            return None;
        }

        let package_type = match &bytes[0..3] {
            b"SYN" => PackageType::SYN,
            b"ACK" => PackageType::ACK,
            b"NAK" => PackageType::NAK,
            b"PKG" => PackageType::PKG,
            b"END" => PackageType::END,
            _ => return None,
        };

        let sequence = bytes[3];
        let data = bytes[4..].to_vec();

        Some(Package {
            package_type,
            sequence,
            data,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.package_type.as_bytes().to_vec();
        bytes.push(self.sequence);
        bytes.extend(&self.data);
        bytes
    }
    
}

#[derive(Debug, PartialEq)]
pub enum PackageType {
    SYN,
    ACK,
    NAK,
    PKG,
    END,
}

impl PackageType {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            PackageType::SYN => b"SYN",
            PackageType::ACK => b"ACK",
            PackageType::NAK => b"NAK",
            PackageType::PKG => b"PKG",
            PackageType::END => b"END",
        }
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageType::SYN => write!(f, "SYN"),
            PackageType::ACK => write!(f, "ACK"),
            PackageType::NAK => write!(f, "NAK"),
            PackageType::PKG => write!(f, "PKG"),
            PackageType::END => write!(f, "END"),
        }
    }
}

