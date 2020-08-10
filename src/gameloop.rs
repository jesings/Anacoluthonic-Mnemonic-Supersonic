use std::net::{IpAddr,SocketAddr};
use std::time::{Duration, Instant};
use std::io::{Read};
use std::collections::HashMap;
use std::fs::read_dir;
use std::sync::{Arc,Mutex};
use std::thread;
use sdl2::pixels::Color;

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;
#[path = "menu.rs"] mod menu;
#[path = "server.rs"]mod server;
#[path = "skill.rs"]pub mod skill;

mod client;
mod gamestate;
mod render;
mod console;

static FRAMERATE: u32 = 60;

pub fn gameloop(addr:String) {
    
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

    let mainmenu = gamestate::MenuItems {
      name: "Main menu".to_string(),
      buttons: vec!(
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy:  0.7, text: "Start Game".to_string(), font: "Inconsolata".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::gotogame},
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy:  0.77, text: "Settings".to_string(), font: "Inconsolata".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::fdummy}
        ),
      sliders: vec!(),
    };

    let gd = Arc::new(Mutex::new(gamestate::GameData {
        players: Vec::new(),
        grid: None,
        pid: 0 as usize,
        flag: false,
    }));
    
    let mut gs = gamestate::GameState{
        canvas: canvas,
        pump: sdl_context.event_pump().unwrap(),
        console: None,
        fonts: font_hash,
        vidsub: video_subsystem,
        scene: gamestate::Scenes::Menu(mainmenu),
        gamedata: Arc::clone(&gd),
    };


    // to be called when connecting from menu
    client::connect(gd, addr);
    
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
