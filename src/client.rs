use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::io::{Read,Write,Error};

pub fn connect(m: String) -> Result<bool, Error>{
    let ip:SocketAddr;
    ip=SocketAddr::new(m.parse::<IpAddr>().unwrap(),54952);
    println!("Attempting to connect to {}",ip);
    let mut stream:TcpStream;
    stream = TcpStream::connect(ip)?;
    let mut buf=[0;10];
    stream.read(&mut buf)?;
    println!("{:?}",buf);
    Ok(true)
}
