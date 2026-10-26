
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

fn main() {
    
    // This defines the multicast listening port to the sddp packets
    let multicast_listeneing_ip: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
    let multicast_port:u16 = 1902;

    // Converting seperate ip and port to a single ipv4 socekt type
    let multicast_socket: SocketAddrV4 = SocketAddrV4::new(multicast_listeneing_ip, multicast_port);

    // Create a UDP socket 
    let udp_listening_socket = UdpSocket::bind(multicast_socket).expect("Unable to bind to the socket");

    // first join to the multicast group
    udp_listening_socket.join_multicast_v4(&multicast_listeneing_ip, &Ipv4Addr::UNSPECIFIED).expect("Unable to join to the multicast group");

    // Block until recive
    let mut buffer = [0u8; 1024];
    let (len, src) = udp_listening_socket.recv_from(&mut buffer).expect("Unable to receive");
      
    println!("📨 Received {} bytes from {}", len, src);
    println!("📝 Data: {}", String::from_utf8_lossy(&buffer[..len]));

    println!("Ending operation");


}


