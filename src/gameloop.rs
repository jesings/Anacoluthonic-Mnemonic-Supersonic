use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::time::{Duration, Instant};
use std::io::{Read,Write};
use std::collections::HashMap;
use std::fs::read_dir;
use sdl2::ttf::Font;

use sdl2::ttf::init;

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;
#[path = "menu.rs"] mod menu;
mod gamestate;
mod render;
mod event;
mod console;

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


    let mut font_hash = HashMap::new();

    //Note, requires cargo to be run from project root, but we'll deal with it later
    let paths = read_dir("data/fonts/").unwrap();
    for path in paths {
        let name = path.unwrap().path();
        let ext = match name.extension() {
            Some(g) => {g},
            None => continue,
        };
        if ext == "otf" || ext == "ttf" {
            let key = name.file_stem().unwrap().to_str().unwrap();
            let value = ttf_context.load_font(name.clone(), 12).unwrap();
            font_hash.insert(String::from(key), value);
            println!("Font added: {}", key);
        }
    }


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
        console: None,
        fonts: font_hash,
        vidsub: video_subsystem,
        scene: gamestate::Scenes::GamePlay(gd),
    };

    'running: loop {
        let begin = Instant::now();

        //match gs.handle_events() {
        //    false => break 'running,
        //    true => {},
        //}

        match gs.update() {
            true => {},
            false => break 'running,
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
