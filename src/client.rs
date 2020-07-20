use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::io::{Read,Write,Error};

pub fn connect(m: String) -> Result<TcpStream, Error>{
    let ip:SocketAddr;
    ip=SocketAddr::new(m.parse::<IpAddr>().expect("thats not an ip address holy shit im freaking out"),54952);
    println!("Attempting to connect to {}",ip);
    TcpStream::connect(ip)
}
