use mirror_image::protocol::Protocol;

use std::collections::HashMap;
use std::io;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:6969")?;
    println!("Server listening on {}", socket.local_addr().expect("couldn't get local address"));

    let protocols = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let mut buf = [0; 1024];
        let (size, client_addr) = socket.recv_from(&mut buf)?;
        let request_packet = buf[..size].to_owned();

        let protocols = Arc::clone(&protocols);
        let socket = socket.try_clone()?;

        thread::spawn(move || {
            let mut thread_protocols = protocols.lock().unwrap();
            let protocol = thread_protocols.entry(client_addr).or_insert_with(Protocol::new);

            print!("Received request from {}: ", client_addr);

            match protocol.handle_request(socket, client_addr, &request_packet) {
                Ok(_) => {
                    println!("Request handled successfully!");
                }
                Err(err) => {
                    eprintln!("Error handling request: {}", err);
                }
            }
        });
    }
}




// use mirror_image::client_protocol::ClientProtocol;
// use mirror_image::protocol::Protocol;
// use mirror_image::Package;
// //use mirror_image::client_protocol::ClientProtocolVec;
// use mirror_image::teste;

// use std::clone;
// use std::collections::HashMap;
// use std::io;
// use std::net::UdpSocket;
// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};
// use std::thread;

// /* 
// pub fn main() {
//     let (sender, receiver): (Sender<Package>, Receiver<Package>) = mpsc::channel();
//     let mut udpserver = UdpServer::new("127.0.0.1:6969", sender);
//     let _ = udpserver.spawn_server();
//     teste::run(receiver);
// }*/


// fn main() -> io::Result<()> {
//     let socket = UdpSocket::bind("127.0.0.1:6969")?;
//     println!("Server listening on {}", socket.local_addr().expect("couldn't get local address"));

//     // vou ter q instanciar um array de protocolos, pois eu preciso persistir o estado de cada cliente
//     // então tenho que associar cliente a protocol verificando o endereço do cliente e quando receber um pacote
//     // END eu removo o protocolo associado a esse cliente

//     let protocols = Arc::new(Mutex::new(HashMap::new()));
//     let cloned_protocols = protocols.clone();

//     loop {
//         let mut buf = [0; 1024];
//         let (size, client_addr) = socket.recv_from(&mut buf)?;
//         let request_packet = &buf[..size];

//         let cloned_socket = socket.try_clone()?;
//         let cloned_request_packet = request_packet.to_owned();
//         let mut thread_protocols = cloned_protocols.lock().unwrap();

//         let mut protocol = thread_protocols.entry(client_addr).or_insert(Protocol::new());

//         print!("Received request from {}: ", client_addr);

//         let handle = thread::spawn(move || {
//             match protocol.handle_request(cloned_socket, client_addr, &cloned_request_packet) {
//                 Ok(_) => {
//                     println!("Request handled successfully!");
//                 }
//                 Err(err) => {
//                     eprintln!("Error handling request: {}", err);
//                 }
//             }
//         });
        
//         let mut thread_protocols2 = cloned_protocols.lock().unwrap();
//         thread_protocols2.insert(client_addr, protocol.to_owned());

//         std::mem::forget(handle);
//     }
// }
