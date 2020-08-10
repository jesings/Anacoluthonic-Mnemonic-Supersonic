use std::net::{TcpStream,UdpSocket,SocketAddr,IpAddr};
use std::io::{Error,Read};
use std::sync::{Mutex,Arc};
use std::thread;
use std::time::Duration;

use super::gamestate::{GameData};
use super::entities::*;
use super::server::PORT;
use super::grid::*;
    
pub fn connect(gd: Arc<Mutex<GameData>>, addr: &String) {
    let sip:SocketAddr = SocketAddr::new(addr.parse::<IpAddr>().expect("thats not an ip address holy shit im freaking out"),PORT);
    println!("Attempting to connect to {}",sip);
    let mut stream = TcpStream::connect(sip).expect("could not connect to server");

    let mut pidbuf: [u8; 2]= [0; 2];
    let mut buf: [u8; 16]= [0; 16];
    stream.read(&mut pidbuf).expect("no pid");
    let pid:u8 = pidbuf[0];
    let pln:u8 = pidbuf[1];
    stream.read(&mut buf).expect("no seed recieved");
    let seed:u128 = u128::from_le_bytes(buf);
    println!("pid: {}/{} seed: {:X} ({:?})",pid,pln,seed,stream);
    let a:SocketAddr =  stream.local_addr().unwrap();
    {
        let mut gdata = gd.lock().unwrap();
        gdata.ingame = true;
        gdata.pid = pid as usize;
        gdata.players.clear();
        for _ in 0..pln {
            gdata.players.push(Player::new());
        }
        gdata.grid = match Grid::random_grid(400, 400, seed) {
            Ok(g) => Some(g),
            Err(_e) => panic!("aaaaaa the random grid didnt get generated???"),
        };
    }
    thread::spawn(move || {
        clientThread(gd,a,sip);
    });
}

pub fn clientThread(a: Arc<Mutex<GameData>>, aaa: SocketAddr, sip: SocketAddr){
    let mut posbuf: [u8; 17] = [0; 17];
    let mut udps = UdpSocket::bind(aaa).expect("could not bind udp port!!!");
    udps.set_read_timeout(Some(Duration::new(2,0)));
    'running: loop{
        //println!("waiting for server to send to {}", aaa);
        match udps.recv_from(&mut posbuf){
            Ok(_)=>{
                //println!("recieved {:?}, waiting for mut",&posbuf);
                let mut q=a.lock().unwrap();
                setposbuf(&posbuf, &mut q.players);
            },
            Err(e)=>{eprintln!("{}",e);},
        };
        let mut q=a.lock().unwrap();
        if !q.ingame {
            break 'running;
        }
        if q.flag {
            getposbuf(q.pid as u8, &q.players[q.pid], &mut posbuf);
            let udps = udps.try_clone().expect("idk couldnt clone");
            thread::spawn(move || {udps.send_to(&posbuf,sip);});
            q.flag = false;
        }
    }
}
