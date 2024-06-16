
#[cfg(test)]
mod tests {
    use mirror_image::PackageType;
    use std::net::UdpSocket;
    use mirror_image::Package;

    #[test]
    fn test_handle_request_syn() {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
        // socket.connect("127.0.0.1:6969").expect("couldn't connect to address");

        // assert_eq!(socket.peer_addr().unwrap(),
        //    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6969)));

        let request_packet = Package::new(socket.local_addr().expect("req pack error")).syn().to_bytes();

        let teste = socket.send_to(&request_packet, "127.0.0.1:6969").expect("couldn't send data");

        let mut buf = [0; 1024];
        let (_, addr) = socket.recv_from(&mut buf).unwrap();

        let response_packet = Package::from_bytes(&buf, socket.local_addr().expect("req pack error")).unwrap();
        
        let pkg = Package::new(socket.local_addr().expect("req pack error")).new_data(1, [1, 2, 3, 4].as_ref());
        socket.send_to(&pkg.to_bytes(), "127.0.0.1:6969").expect("couldn't send data");
        
        assert_eq!(response_packet.package_type, PackageType::ACK, "Expected ACK packet");
    }
}
