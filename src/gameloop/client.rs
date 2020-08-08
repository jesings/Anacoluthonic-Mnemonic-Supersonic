use std::net::{TcpStream,UdpSocket,SocketAddr};
use std::io::Error;
use std::sync::{Mutex,Arc};
use std::thread;

use super::gamestate::GameData;
use super::entities::{setposbuf,getposbuf};

pub fn connect(ip: SocketAddr) -> Result<TcpStream, Error>{
    println!("Attempting to connect to {}",ip);
    TcpStream::connect(ip)
}

pub fn clientThread(a: Arc<Mutex<GameData>>, aaa: SocketAddr, sip: SocketAddr){
    let mut posbuf: [u8; 17] = [0; 17];
    let mut udps = UdpSocket::bind(aaa).expect("could not bind udp port!!!");
    
    'running: loop{
        //println!("waiting for server to send to {}", aaa);
        match udps.recv_from(&mut posbuf){
            Ok(_)=>{
                //println!("recieved {:?}, waiting for mut",&posbuf);
                let mut q=a.lock().unwrap();
                setposbuf(&posbuf, &mut q.players);
                if q.flag {
                    getposbuf(q.pid as u8, &q.players[q.pid], &mut posbuf);
                    let udps = udps.try_clone().expect("idk couldnt clone");
                    thread::spawn(move || {udps.send_to(&posbuf,sip);});
                    q.flag = false;
                }
            },
            Err(e)=>{eprintln!("{}",e);},
        };
    }
}
