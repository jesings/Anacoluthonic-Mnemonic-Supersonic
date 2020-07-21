use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::io::{Read,Write,Error};

pub fn connect(ip: SocketAddr) -> Result<TcpStream, Error>{
    println!("Attempting to connect to {}",ip);
    TcpStream::connect(ip)
}
