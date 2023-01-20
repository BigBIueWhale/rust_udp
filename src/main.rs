// Use Tokio to send a UDP packet to a remote host

use tokio::net::UdpSocket;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Send the message: "Hello, world!" to the IPv6 address: 2001:db8::1 port 8080
    let addr = "[2a02:14f:1:ae9e:2065:296a:d508:1b3a]:8080".parse::<SocketAddr>()?;
    println!("Sending to: {}", addr);
    let socket = UdpSocket::bind("[0:0:0:0:0:0:0:0]:0").await?;
    socket.send_to(b"Hello, world!", &addr).await?;
    Ok(())
}
