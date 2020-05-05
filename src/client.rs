use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::io::{Read,Write};

pub fn connect(m: String){
	let ip:SocketAddr;
	match m.parse::<IpAddr>(){
		Ok(q)=>ip=SocketAddr::new(q,54952),
		Err(e)=>{eprintln!("{}",e);return},
	}
	println!("Attempting to connect to {}",ip);
	let mut stream:TcpStream;
	match TcpStream::connect(ip){
		Ok(q)=>stream=q,
		Err(e)=>{eprintln!("{}",e);return},
	}
	let mut buf=[0;10];
	stream.read(&mut buf);
	println!("{:?}",buf);
}
