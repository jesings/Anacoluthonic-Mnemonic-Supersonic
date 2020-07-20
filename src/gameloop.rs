use std::net::{TcpListener,TcpStream,UdpSocket,IpAddr,Ipv4Addr,SocketAddr};
use std::time::{Duration, Instant};
use std::io::{Read,Write};
use std::collections::HashMap;
use std::fs::read_dir;
use std::process::Command;
use sdl2::pixels::Color;

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;
#[path = "menu.rs"] mod menu;
#[path = "client.rs"]mod client;
mod gamestate;
mod render;
//mod event;
mod console;

static FRAMERATE: u32 = 60;

pub fn gameloop(addr:String) {
    let mut stream = client::connect(addr).expect("could not connect to server"); // comment for the rendering and stuff or whatever

    let mut buf: [u8; 16]= [0; 16];
    stream.read(&mut buf).expect("no seed recieved");
    let seed:u128 = u128::from_le_bytes(buf);
    println!("seed: {}",seed);
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");

    let window = video_subsystem.window("Anacoluthonic Mnemonic Supersonic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();


    let mut font_hash = HashMap::new();

    let paths = read_dir("data/fonts/").unwrap();
    for path in paths {
        let name = path.unwrap().path();
        let ext = match name.extension() {
            Some(g) => {g},
            None => continue,
        };
        if ext == "otf" || ext == "ttf" {
            let key = name.file_stem().unwrap().to_str().unwrap();
            let mut value = ttf_context.load_font(name.clone(), 256).unwrap();
            value.set_kerning(true);
            font_hash.insert(String::from(key), value);
            println!("Font added: {}", key);
        }
    }


    let mut grid: grid::Grid;
    match grid::Grid::random_grid(400, 400, seed) {
        Ok(g) => grid = g,
        Err(_e) => return,
    }

    let mut mainmenu = gamestate::MenuItems {
      name: "Main menu".to_string(),
      buttons: vec!(menu::Button {height: 0.1, width: 0.8, cx: 0.5, cy:  0.5, text: "Test".to_string(), font: "Inconsolata".to_string(), textcolor: Color::RGB(0, 255, 0)}),
      sliders: vec!(),
    };

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
        //scene: gamestate::Scenes::GamePlay(gd),
        scene: gamestate::Scenes::Menu(mainmenu),
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
