use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,SocketAddr};
use std::io::{Read,Write};
use std::process::Command;
use std::convert::TryInto;

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;

static PLAYERS:u8 = 2; // should be configurable at server creation later
pub static PORT:u16 = 54952;

pub fn localip()->Result<IpAddr,std::io::Error>{
    let output = if cfg!(target_os="windows"){
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"imagine using windows"))
    }else{
        Command::new("sh").arg("-c").arg("ip address show | grep 192\\.168\\.[0-9]*\\.[0-9]* -ao --color=never | head -1 | tr -d [:space:]").output()?
    };
    let outstr = match String::from_utf8(output.stdout){
        Ok(q)=>q,
        Err(_)=>{return Err(std::io::Error::new(std::io::ErrorKind::Other,"couldnt convert ipaddr string for some reason"));},
    };
    Ok(outstr.parse::<IpAddr>().unwrap())
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
    let a:SocketAddr = match localip(){
        Ok(q)=>SocketAddr::new(q,PORT),
        Err(e)=>{eprintln!("{}",e);return},
    };
    println!("binding to {}",a);
    let listener:TcpListener = match TcpListener::bind(a){
        Ok(q)=>q,
        Err(e)=>{eprintln!("{}",e);return},
    };
    let mut sss: Vec<TcpStream> = Vec::new();
    let mut adr: Vec<SocketAddr> = Vec::new();
    let mut players: Vec<entities::Player> = Vec::new();
    for i in 0..PLAYERS{
        match listener.accept(){
            Ok((q,u))=>{
                //println!("{:?}",q);
                sss.push(q);
                adr.push(u);},
            Err(_)=>{},
        };
    }
    for (i,s) in sss.iter().enumerate(){ // initial data dump
        connect(s,seed,i as u8);
        players.push(entities::Player::new());
    }
    drop(sss);
    let mut udps = UdpSocket::bind(a).expect("COULD NOT BIND UDP PORT!!!!!!");
    let mut posbuf: [u8; 17] = [0; 17];
    'running: loop {
        let au:SocketAddr = udps.recv_from(&mut posbuf).expect("packet reception error").1;
        entities::setposbuf(&posbuf, &mut players);
        let c = posbuf[0];
        for (i,pl) in players.iter().enumerate(){
            let b = i as u8;
            if b!=c {
                entities::getposbuf(b,pl,&mut posbuf);
                //println!("sent {:?} to player {} at {}",&posbuf,b,au);
                udps.send_to(&posbuf,au);
            }
        }
    }
}

fn connect(mut s: &TcpStream, seed: u128, pid: u8){
    //println!("{:?}",s);
    s.write(&[pid,PLAYERS]);
    s.write(&(seed.to_le_bytes()));
}
