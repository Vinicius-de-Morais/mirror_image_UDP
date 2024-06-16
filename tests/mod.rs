use std::net::{Ipv4Addr, SocketAddrV4};


#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, UdpSocket};

    use mirror_image::{handle_request, protocol::Protocol, Package, PackageType};

    use super::*;

    #[test]
    fn test_handle_request_syn() {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
        // socket.connect("127.0.0.1:6969").expect("couldn't connect to address");

        // assert_eq!(socket.peer_addr().unwrap(),
        //    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6969)));

        let request_packet = Package::new_syn().to_bytes();

        let teste = socket.send_to(&request_packet, "127.0.0.1:6969").expect("couldn't send data");

        let mut buf = [0; 1024];
        let (_, addr) = socket.recv_from(&mut buf).unwrap();

        let response_packet = Package::from_bytes(&buf).unwrap();
        
        
        let pkg = Package::new_data(1, [1, 2, 3, 4].as_ref());
        socket.send_to(&pkg.to_bytes(), "127.0.0.1:6969").expect("couldn't send data");
        
        assert_eq!(response_packet.package_type, PackageType::ACK, "Expected ACK packet");
    }

    #[test]
    fn test_handle_request_ack_received() {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let client_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234).into();
        let request_packet = Package::new_syn().to_bytes();

        let cloned_socket = socket.try_clone().unwrap();
        let handle_request_thread = std::thread::spawn(move || {
            handle_request(cloned_socket, client_addr, &request_packet).unwrap();
        });

        let mut buf = [0; 1024];
        let (_, addr) = socket.recv_from(&mut buf).unwrap();
        assert_eq!(addr, client_addr);
        let response_packet = Package::from_bytes(&buf).unwrap();
        assert_eq!(response_packet.package_type, PackageType::ACK, "Expected ACK packet");

        handle_request_thread.join().unwrap();
    }

    // #[test]
    // fn test_handle_request_syn() {
    //     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    //     let client_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234).into();
    //     let request_packet = Package::new_syn().to_bytes();
    
    //     handle_request(socket, client_addr, &request_packet).unwrap();
    
    //     // Assert that an ACK packet was sent to the client
    //     let mut buf = [0; 1024];
    //     let socket_recv = UdpSocket::bind("127.0.0.1:0").unwrap();
    //     let (_, addr) = socket_recv.recv_from(&mut buf).unwrap();
    //     assert_eq!(addr, client_addr);
    //     let response_packet = Package::from_bytes(&buf).unwrap();
    //     assert_eq!(response_packet.package_type, PackageType::ACK, "Expected ACK packet");
    // }

    // #[test]
    // fn test_handle_request_invalid_package_type() {
    //     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    //     let client_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234).into();
    //     let request_packet = Package::new_data(1, b"test").to_bytes();
        
    //     handle_request(socket.try_clone().unwrap(), client_addr, &request_packet).unwrap();

    //     // Assert that a NAK packet was sent to the client
    //     let mut buf = [0; 1024];
    //     let (_, addr) = socket.recv_from(&mut buf).unwrap();
    //     assert_eq!(addr, client_addr);
    //     let response_packet = Package::from_bytes(&buf).unwrap();
    //     assert_eq!(response_packet.package_type, PackageType::NAK, "Expected NAK packet");
    // }

    // #[test]
    // fn test_handle_request_receive_file() {
    //     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    //     let client_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234).into();
    //     let mut request_packet = Package::new_syn().to_bytes();
    //     socket.send_to(&request_packet, client_addr).unwrap();

    //     // Receive ACK packet
    //     let mut buf = [0; 1024];
    //     let (_, _) = socket.recv_from(&mut buf).unwrap();

    //     // Send data packets
    //     let data_packets = [
    //         Package::new_data(1, b"Hello"),
    //         Package::new_data(2, b"World"),
    //         Package::new_data(3, b"!"),
    //         Package::new_data(4, b"END"),
    //     ];
    //     for packet in &data_packets {
    //         request_packet = packet.to_bytes();
    //         socket.send_to(&request_packet, client_addr).unwrap();
    //     }

    //     handle_request(socket, client_addr, &request_packet).unwrap();

    //     // Assert that the file was received successfully
    //     let file_contents = std::fs::read_to_string("received_file.txt").unwrap();
    //     assert_eq!(file_contents, "HelloWorld!", "Expected file contents: 'HelloWorld!'");
    // }
}
