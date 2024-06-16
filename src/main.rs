use std::net::UdpSocket;
use std::io;
use std::thread;
use std::net::{IpAddr, Ipv4Addr};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:6969")?;
    //println!("Server listening on {}", socket.local_addr()?);

    // vou ter q instanciar um array de protocolos, pois eu preciso persistir o estado de cada cliente
    // então tenho que associar cliente a protocol verificando o endereço do cliente e quando receber um pacote 
    // END eu removo o protocolo associado a esse cliente

    loop {
        let mut buf = [0; 1024];
        let (size, client_addr) = socket.recv_from(&mut buf)?;
        let request_packet = &buf[..size];
    
        let cloned_socket = socket.try_clone()?;
        let cloned_request_packet = request_packet.to_owned();

        print!("Received request from {}: ", client_addr);

        let handle = thread::spawn(move || {

            if let Err(err) = mirror_image::handle_request(cloned_socket, client_addr, &cloned_request_packet) {
                eprintln!("Error handling request: {}", err);
            }
        });
    
        std::mem::forget(handle);
    }
}