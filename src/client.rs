use std::net::{TcpStream,UdpSocket,SocketAddr,IpAddr};
use std::io::{Read};
use std::sync::{Mutex,Arc};
use std::thread;
use std::time::Duration;

use super::gamestate::GameData;
use super::entities::*;
use super::{server::PORT,packet::*};
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
        gdata.buf[0] = pid;
        gdata.buf[1] = 0;
        gdata.bufpos = 1;
        gdata.players.clear();
        for _ in 0..pln {
            gdata.players.push(Player::new());
        }
        gdata.grid = Some(Grid::random_grid(400, 400, seed).expect("aaaaaa the random grid didnt get generated???"));
    }
    thread::spawn(move || {
        client_thread(gd,a,sip);
    });
}

pub fn client_thread(a: Arc<Mutex<GameData>>, aaa: SocketAddr, sip: SocketAddr){
    let mut buf: [u8; 4096] = [0; 4096];
    let udps = UdpSocket::bind(aaa).expect("could not bind udp port!!!");
    udps.set_read_timeout(Some(Duration::new(0,20_000000))).expect("Cannot go into timeout, no dessert for a week young lady");
    'running: loop{
        //println!("waiting for server to send to {}", aaa);
        match udps.recv_from(&mut buf){
            Ok(_)=>{
                //println!("recieved {:?}, waiting for mut",&buf);
                packet_decode(&buf, Arc::clone(&a));
            },
            Err(_e)=>{/*eprintln!("{}",e);*/},
        };
        let mut q = a.lock().unwrap();
        if !q.ingame {
            break 'running;
        }
        if q.bufpos>1 {
            let cpy = q.bufpos;
            q.buf[cpy] = 0; // ensure packet decode terminates properly
            match udps.send_to(&q.buf,sip) {
                Ok(_) => {q.bufpos = 1;},
                Err(_) => {},
            };
        }
    }
}
