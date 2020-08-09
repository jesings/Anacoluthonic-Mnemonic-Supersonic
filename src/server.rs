use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,SocketAddr};
use std::io::{Read,Write};
use std::process::Command;
use std::thread;
use std::sync::{Arc,Mutex};
use std::time::Duration;
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
    let mut players: Vec<entities::Player> = Vec::new();
    for i in 0..PLAYERS{
        match listener.accept(){
            Ok((q,u))=>{
                //println!("{:?}",q);
                sss.push(q);
                if u.ip() == ipa {adr.push(u)} else {adr.push(SocketAddr::new(u.ip(),PORT))};},
            Err(_)=>{},
        };
    }
    println!("{:?}",adr);
    for (i,s) in sss.iter().enumerate(){ // initial data dump
        connect(s,seed,i as u8);
        players.push(entities::Player::new());
    }
    drop(sss);
    let pdata = Arc::new(Mutex::new(players));
    let mut udps = UdpSocket::bind(a).expect("COULD NOT BIND UDP PORT!!!!!!");
    let mut posbuf: [u8; 17] = [0; 17];
    {
        let pdata = Arc::clone(&pdata);
        let udps = udps.try_clone().expect("coppuldnt get socket clone");
        thread::spawn(move || {serverRecieve(pdata,udps)});
    }
    'running: loop {
        for i in 0..PLAYERS{
            let b = i as usize;
            {
                let pl = pdata.lock().unwrap();
                entities::getposbuf(i,&pl[b],&mut posbuf);
            }
            for j in 0..PLAYERS {
                if i!=j {
                    println!("trying to send to {}, pid {}", adr[j as usize], i);
                    match udps.send_to(&posbuf,adr[j as usize]){
                        Err(e)=>{eprintln!("{}",e);},
                        _=>{},
                    };
                }
            }
        }
    }
}


fn connect(mut s: &TcpStream, seed: u128, pid: u8){
    let mut vectoappend = vec!();
    vectoappend.extend_from_slice(&[pid,PLAYERS]);
    vectoappend.extend_from_slice(&seed.to_le_bytes());
    s.write(vectoappend.as_slice());
}

fn serverRecieve(pdata: Arc<Mutex<Vec<entities::Player>>>, udps: UdpSocket){
    udps.set_read_timeout(Some(Duration::new(5,0)));
    let mut posbuf: [u8; 17] = [0; 17];
    'running: loop {
        match udps.recv_from(&mut posbuf){
            Ok(_)=>{},
            Err(_)=>{break 'running;},
        };
        let mut players = pdata.lock().unwrap();
        entities::setposbuf(&posbuf, &mut *players);
    }
    println!("server recieved literally zero data");
}
