use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::time::{Duration, Instant};
use std::io::{Read,Write};
use sdl2::ttf::init;

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;
#[path = "menu.rs"] mod menu;
mod gamestate;
mod render;
mod event;

static FRAMERATE: u32 = 60;

fn listen(){
    let listener:TcpListener;
    match TcpListener::bind("127.0.0.1:54952"){
        Ok(q)=>listener=q,
        Err(_e)=>return,
    }
    for s in listener.incoming(){
        match s{
            Ok(ss)=>connect(ss),
            Err(_e)=>{},
        }
    }
}

fn connect(mut stream: TcpStream){
    stream.write(&[69]);
}

pub fn gameloop() {
    //listen(); // comment for the rendering and stuff or whatever
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Anacoluthonic Mnemonic Supersonic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let mut grid: grid::Grid;
    match grid::Grid::random_grid(400, 400) {
        Ok(g) => grid = g,
        Err(_e) => return,
    }

    let mut gd = gamestate::GameData {
        player: entities::Player::new(),
        grid: grid,
    };

    let mut gs = gamestate::GameState{
        canvas: canvas,
        pump: sdl_context.event_pump().unwrap(),
        console: false,
        scene: gamestate::Scenes::GamePlay(gd),
    };

    'running: loop {
        let begin = Instant::now();

        match gs.handle_events() {
            false => break 'running,
            true => {},
        }

        match gs.update() {
            true => {},
            false => {},
        }

        match gs.render() {
            Ok(_r) => {},
            Err(e) => eprintln!("{}", e),
        }


        let delta = begin.elapsed();

        let idle = 1_000_000_000u32 / FRAMERATE - delta.as_millis() as u32;

        std::thread::sleep(Duration::new(0, idle));
    }
}
