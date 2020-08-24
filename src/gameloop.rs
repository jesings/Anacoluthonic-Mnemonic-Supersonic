//use std::net::{IpAddr,SocketAddr};
use std::time::{Duration, Instant};
//use std::io::{Read};
use std::collections::HashMap;
use std::fs::read_dir;
use std::sync::{Arc,Mutex};
//use std::thread;
use sdl2::pixels::Color;

#[path = "grid.rs"] pub mod grid;
#[path = "entities.rs"] pub mod entities;
use entities::{Entity};
#[path = "menu.rs"] mod menu;
#[path = "hud.rs"] mod hud;
#[path = "server.rs"] pub mod server;
#[path = "skill.rs"] pub mod skill;
#[path = "client.rs"] mod client;
#[path = "packet.rs"] pub mod packet;
#[path = "class.rs"] mod class;

pub mod gamestate;
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
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

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
            for fontsize in 1..=100 {
              let mut value = ttf_context.load_font(name.clone(), fontsize).unwrap();
              value.set_kerning(true);
              value.set_hinting(sdl2::ttf::Hinting::Mono);
              font_hash.insert(format!("{}{}", key, fontsize), value);
            }
            println!("Font added: {}", key);
        }
    }

    let mainmenu = gamestate::MenuItems {
      name: "Main menu".to_string(),
      buttons: vec!(
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy: 0.7, text: "Start Game".to_string(), font: "Inconsolata100".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::gotogame},
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy: 0.77, text: "Settings".to_string(), font: "Inconsolata100".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::fdummy}
        ),
      sliders: vec!(),
    };

    let gd = Arc::new(Mutex::new(gamestate::GameData {
        players: Vec::new(),
        grid: None,
        pid: 0,
        buf: [0; 4096],
        bufpos: 1,
        ingame: false,
    }));
    
    let mut gs = gamestate::GameState{
        canvas: canvas,
        pump: sdl_context.event_pump().unwrap(),
        console: None,
        fonts: font_hash,
        vidsub: video_subsystem,
        scene: gamestate::Scenes::Menu(mainmenu),
        class: Some(class::Class::new(0)), // class picker menu eventually
        huditems: vec!(
            hud::HudItem{height: 120, width: 80, xpadding: 10, ypadding: -10, bgcolor: Color::RGBA(200, 60, 100, 200)},
        ),
        hudtexts: vec!(
            hud::HudText{height: 30, width: 20, xpadding: 10, ypadding: -10, textgen: |gd| {format!("{}", gd.pid)}, color: Color::RGB(0, 255, 0), font: "Inconsolata19".to_string()},
            hud::HudText{height: 30, width: 120, xpadding: 100, ypadding: -10, textgen: |gd| {
              let player = &gd.players[gd.pid];
              format!("HP: {}/{}", player.health() as i32, player.maxhealth() as i32)
            }, color: Color::RGB(255, 0, 0), font: "Inconsolata19".to_string()},
            hud::HudText{height: 30, width: 150, xpadding: -10, ypadding: -10, textgen: |gd| {
                let pos = gd.players[gd.pid].pos();
                format!("{:.2},{:.2}", pos.x, pos.y)
            }, color: Color::RGB(0, 255, 255), font: "Inconsolata19".to_string()},
        ),
        address: addr,
        gamedata: Arc::clone(&gd),
    };


    let mut now: Duration = Duration::new(0,0);
    'running: loop {
        let begin = Instant::now();

        //match gs.handle_events() {
        //    false => break 'running,
        //    true => {},
        //}

        match gs.update(now) {
            true => {},
            false => break 'running,
        }

        match gs.render() {
            Ok(_r) => {},
            Err(e) => eprintln!("{}", e),
        }

        let delta = begin.elapsed();

        let idle = Duration::new(0, 1_000_000_000 / FRAMERATE - delta.as_millis() as u32);
        now += idle;
        std::thread::sleep(idle);
    }
}
