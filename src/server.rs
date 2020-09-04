use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,SocketAddr};
use std::io::{Write};
use std::process::Command;
//use std::thread;
use std::sync::{Arc,Mutex};
use std::time::Duration;

use crate::gameloop::grid::*;
use crate::gameloop::entities::*;
use crate::gameloop::packet::*;
use crate::gameloop::gamestate::*;

static PLAYERS:u8 = 2; // should be configurable at server creation later
pub static PORT:u16 = 54952;

pub fn localip()->Result<IpAddr,std::io::Error>{
    let output = if cfg!(target_os="windows"){
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"imagine using windows"))
    }else{
        Command::new("sh").arg("-c").arg("ip address show | grep -v docker | grep -v tun | grep \"inet [0-9]*\\.[0-9]*\\.[0-9]*\\.[0-9]*\" -ao --color=never | tail -1 | tr -d [inet][:space:]").output()?
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
    println!("seed: {:X}",seed);
    let ipa = localip().expect("couldnt get local ip");
    let a:SocketAddr = SocketAddr::new(ipa,PORT);
    //let a2:SocketAddr = SocketAddr::new(ipa,PORT+1);
    println!("binding to {}",a);
    let listener:TcpListener = match TcpListener::bind(a){
        Ok(q)=>q,
        Err(e)=>{eprintln!("{}",e);return},
    };
    let mut sss: Vec<TcpStream> = Vec::new();
    let mut adr: Vec<SocketAddr> = Vec::new();
    let mut players: Vec<Player> = Vec::new();
    for _i in 0..PLAYERS{
        match listener.accept(){
            Ok((q,u))=>{
                //println!("{:?}",q);
                sss.push(q);
                adr.push(u);
            },
            Err(_)=>{},
        };
    }
    //println!("{:?}",adr);
    for (i,s) in sss.iter().enumerate(){ // initial data dump
        connect(s,seed,i as u8);
        players.push(Player::new());
    }
    let gdata = Arc::new(Mutex::new(GameData {
        players: players,
        tickents: Vec::new(),
        grid: Some(Grid::new_from_roomgen(400, 400, seed).expect("aaaaaa the random grid didnt get generated???")),
        pid: 0,
        buf: [0; 4096],
        bufpos: 1,
        ingame: true,
    }));
    let udps = UdpSocket::bind(a).expect("COULD NOT BIND UDP PORT!!!!!!");
/*    {
        let gdata = Arc::clone(&gdata);
        thread::spawn(move || {tcp_ping(gdata)});
    }*/
    udps.set_read_timeout(Some(Duration::new(20,0))).expect("Cannot go into timeout, no dessert for you young man");
    let mut buf: [u8; 4096] = [0; 4096];
    'running: loop {
        match udps.recv_from(&mut buf){
            Ok(_)=>{},
            Err(_)=>{break 'running;},
        };
        packet_decode(&buf, Arc::clone(&gdata));
        for i in 0..PLAYERS {
            if i!=buf[0] {
                //println!("trying to send to {}, pid {}", adr[j as usize], i);
                match udps.send_to(&buf,adr[i as usize]){
                    Err(e)=>{eprintln!("{}",e);},
                    _=>{},
                };
            }
        }
    }
    println!("server recieved literally zero data");
}


fn connect(mut s: &TcpStream, seed: u128, pid: u8){
    let mut vectoappend = vec!();
    vectoappend.extend_from_slice(&[pid,PLAYERS]);
    vectoappend.extend_from_slice(&seed.to_le_bytes());
    s.write(vectoappend.as_slice()).expect("Cannot write to clients, I need to go back to pre-k");
}
/*fn tcp_ping(gdata: Arc<Mutex<GameData>>){
    //todo: should periodically update clients through tcp to prevent desync
    return
}*/
