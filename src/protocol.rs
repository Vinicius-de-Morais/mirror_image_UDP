use std::{fs::File, io::{self, Write}, net::{SocketAddr, UdpSocket}, path};

use crate::{Package, PackageType};
use std::net::{Ipv4Addr, SocketAddrV4};


pub struct Protocol {
    pub packages: Vec<Package>,
    pub current_sequence: u8,
    pub current_ack: u8,
    pub current_nak: u8,
}


impl Protocol {
    pub fn new() -> Protocol {
        Protocol {
            packages: vec![],
            current_sequence: 0,
            current_ack: 0,
            current_nak: 0,
        }
    }

    pub fn handle_request(&mut self, socket: UdpSocket, client_addr: SocketAddr, request_packet: &[u8]) -> io::Result<()> {
        let request_packet = Package::from_bytes(request_packet).unwrap();
        match request_packet.package_type {
            PackageType::SYN => {
                self.handle_syn(socket, client_addr);
            },
            PackageType::PKG => {
                self.handle_pkg(request_packet, socket, client_addr);
            },
            PackageType::END => {
                self.handle_end(socket, client_addr);
            },

            _ => {
                self.send_nak(socket, client_addr);
            },
        }
        Ok(())
    }

    pub fn handle_syn(&mut self, socket: UdpSocket, client_addr: SocketAddr){
      self.send_ack(socket, client_addr);  
    }
    
    pub fn send_ack(&mut self, socket: UdpSocket, client_addr: SocketAddr){
        self.current_ack += 1;
    
        let response_packet = Package::new_ack();
        let response_packet = response_packet.to_bytes();
        socket.send_to(&response_packet, client_addr);
    }
    
    pub fn send_nak(&mut self, socket: UdpSocket, client_addr: SocketAddr){
        self.current_nak += 1;
    
        let response_packet = Package::new_nak();
        let response_packet = response_packet.to_bytes();
        socket.send_to(&response_packet, client_addr);
    }
    
    pub fn handle_pkg(&mut self, request_packet: Package, socket: UdpSocket, client_addr: SocketAddr){
        if request_packet.sequence == self.current_sequence {
            self.packages.push(request_packet);
            self.current_sequence += 1;
            self.send_ack(socket, client_addr);  
        } else {
            self.send_nak(socket, client_addr);
        }
    }
    
    pub fn handle_end(&mut self, socket: UdpSocket, client_addr: SocketAddr){
         // Ordena os pacotes por sequência, se necessário
         self.packages.sort_by_key(|p| p.sequence);

         // Abre/cria um arquivo para escrita
         let path = client_addr.to_string() + "_file.txt";
         let mut file = File::create(path).unwrap();
 
         // Itera sobre os pacotes e escreve seus dados no arquivo
         for package in &self.packages {
             file.write_all(&package.data).unwrap();
         }
 
         // Envia um ACK final para o cliente
         self.send_ack(socket, client_addr);
 
         // Limpa o vetor de pacotes para estar pronto para a próxima transmissão
         self.packages.clear();
    }

    pub fn send_file(&self, socket: UdpSocket, client_addr: SocketAddr) -> io::Result<()> {
        for package in &self.packages {
            let package = package.to_bytes();
            socket.send_to(&package, client_addr)?;
        }
        Ok(())
    }
}