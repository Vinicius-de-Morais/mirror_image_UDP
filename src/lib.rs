use std::io::{self, Write};
use std::{fmt, fs::File, net::UdpSocket};

pub mod protocol;


#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    pub address: std::net::SocketAddr,
    pub package_type: PackageType,
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl Package {

    pub fn new(address: std::net::SocketAddr) -> Package {
        Package {
            address,
            package_type: PackageType::SYN,
            sequence: 0,
            data: vec![],
        }
    }

    pub fn syn(mut self) -> Package{
        self.package_type = PackageType::SYN;
        self
    }

    pub fn ack(mut self) -> Package{
        self.package_type = PackageType::ACK;
        self
    }

    pub fn nak(mut self) -> Package{
        self.package_type = PackageType::NAK;
        self
    }

    pub fn new_data(mut self, sequence: u8, data: &[u8]) -> Package {

        self.package_type =PackageType::PKG;
        self.sequence = sequence;
        self.data = data.to_vec();
        self
    }

    pub fn from_bytes(bytes: &[u8], address: std::net::SocketAddr) -> Option<Package> {
        if bytes.len() < 4 {
            return None;
        }
    
        let package_type = match &bytes[0..3] {
            b"SYN" => PackageType::SYN,
            b"ACK" => PackageType::ACK,
            b"NAK" => PackageType::NAK,
            b"PKG" => PackageType::PKG,
            b"END" => PackageType::END,
            _ => PackageType::END, // Replace `return None;` with a valid package type
        };
    
        let sequence = bytes[3];
        let data = bytes[4..].to_vec();
    
        Some(Package {
            address,
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

#[derive(Debug, PartialEq, Clone, Copy)]
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

