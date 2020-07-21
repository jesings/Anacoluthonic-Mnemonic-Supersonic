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
#[path = "server.rs"]mod server;

mod gamestate;
mod render;
//mod event;
mod console;

static FRAMERATE: u32 = 60;

pub fn gameloop(addr:String) {
    let sip:SocketAddr = SocketAddr::new(addr.parse::<IpAddr>().expect("thats not an ip address holy shit im freaking out"),server::PORT);
    let mut stream = client::connect(sip).expect("could not connect to server");

    let mut buf: [u8; 16]= [0; 16];
    stream.read(&mut buf).expect("no pid");
    let pid:u8 = buf[0];
    let pln:u8 = buf[1];
    stream.read(&mut buf).expect("no seed recieved");
    let seed:u128 = u128::from_le_bytes(buf);
    println!("pid: {}/{} seed: {} ({:?})",pid,pln,seed,stream);
    let iptst = server::localip().expect("could not get localip");
    let a:SocketAddr =  SocketAddr::new(iptst, if iptst==sip.ip() {server::PORT + 1 + pid as u16} else {server::PORT});
    let mut udps = UdpSocket::bind(a).expect("could not bind udp port!!!");
    let mut posbuf: [u8; 17] = [0; 17];
        
    
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
      buttons: vec!(
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy:  0.7, text: "Start Game".to_string(), font: "Inconsolata".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::fdummy},
        menu::Button {height: 0.06, width: 0.5, cx: 0.5, cy:  0.77, text: "Settings".to_string(), font: "Inconsolata".to_string(), textcolor: Color::RGB(255, 255, 255), bgcolor: Color::RGB(20, 60, 100), callback: menu::fdummy}
        ),
      sliders: vec!(),
    };

    let mut pls: Vec<entities::Player> = Vec::new();
    for _ in 0..pln {
        pls.push(entities::Player::new());
    }
    let mut gd = gamestate::GameData {
        players: pls,
        grid: grid,
        pid: pid as usize,
    };

    let mut gs = gamestate::GameState{
        canvas: canvas,
        pump: sdl_context.event_pump().unwrap(),
        console: None,
        fonts: font_hash,
        vidsub: video_subsystem,
        scene: gamestate::Scenes::GamePlay(gd),
        //scene: gamestate::Scenes::Menu(mainmenu),
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

        match &mut gs.scene {
            gamestate::Scenes::GamePlay(q)=>{
                entities::getposbuf(q.pid as u8, &q.players[q.pid], &mut posbuf);
                //println!("sent {:?}",&posbuf);
                udps.send_to(&posbuf,sip);
                for i in 1..pln{
                    match udps.recv_from(&mut posbuf){
                        Ok(_)=>{
                            //println!("recieved {:?}",&posbuf);
                            entities::setposbuf(&posbuf, &mut q.players);},
                        Err(e)=>{eprintln!("{}",e);},
                    };
                }
            },
            _=>{},
        };
        
        match gs.render() {
            Ok(_r) => {},
            Err(e) => eprintln!("{}", e),
        }


        let delta = begin.elapsed();

        let idle = 1_000_000_000u32 / FRAMERATE - delta.as_millis() as u32;

        std::thread::sleep(Duration::new(0, idle));
    }
}
