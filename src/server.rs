use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::io::{Read,Write};
use std::process::Command;

static PLAYERS:u8 = 2;
static PORT:u16 = 54952;

fn localip(port:u16)->Result<SocketAddr,std::io::Error>{
    let output = if cfg!(target_os="windows"){
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"imagine using windows"))
    }else{
        Command::new("sh").arg("-c").arg("ip address show | grep 192\\.168\\.[0-9]*\\.[0-9]* -ao --color=never | head -1 | tr -d [:space:]").output()?
    };
    let outstr = match String::from_utf8(output.stdout){
        Ok(q)=>q,
        Err(_)=>{return Err(std::io::Error::new(std::io::ErrorKind::Other,"couldnt convert ipaddr string for some reason"));},
    };
    Ok(SocketAddr::new(outstr.parse::<IpAddr>().unwrap(),port))
}

fn globalip(){
    let output = if cfg!(target_os="windows"){
        eprintln!("imagine using windows");return
    }else{
        match Command::new("sh").arg("-c").arg("curl ifconfig.me").output(){Ok(q)=>q,Err(_)=>{eprintln!("could not find global ip address");return},}
    };
    let outstr = match String::from_utf8(output.stdout){
        Ok(q)=>q,
        Err(_)=>{eprintln!("could not find global ip address");return},
    };
    println!("global ip address is {}",outstr);
}

pub fn host(){
    globalip();
    let seed:u128 = rand::random::<u128>();
    println!("seed: {}",seed);
    let a:SocketAddr = match localip(54952){
        Ok(q)=>q,
        Err(e)=>{eprintln!("{}",e);return},
    };
    println!("binding to {}",a);
    let listener:TcpListener = match TcpListener::bind(a){
        Ok(q)=>q,
        Err(e)=>{eprintln!("{}",e);return},
    };
    
    for s in listener.incoming(){
        match s{
            Ok(ss)=>connect(ss,seed),
            Err(e)=>{eprintln!("{}",e);},
        };
    }
}

fn connect(mut s: TcpStream, seed:u128){
    println!("{:?}",s);
    s.write(&(seed.to_le_bytes()));
}
