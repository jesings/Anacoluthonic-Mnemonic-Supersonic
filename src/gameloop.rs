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
mod gamestate;
mod render;
//mod event;
mod console;

static FRAMERATE: u32 = 60;

fn localip()->Result<SocketAddr,std::io::Error>{
    let output = if cfg!(target_os="windows"){
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"imagine using windows"))
    }else{
        Command::new("sh").arg("-c").arg("ip address show | grep 192\\.168\\.[0-9]*\\.[0-9]* -ao --color=never | head -1 | tr -d [:space:]").output()?
    };
    let outstr = match String::from_utf8(output.stdout){
        Ok(q)=>q,
        Err(_)=>{return Err(std::io::Error::new(std::io::ErrorKind::Other,"couldnt convert ipaddr string for some reason"));},
    };
    Ok(SocketAddr::new(outstr.parse::<IpAddr>().unwrap(),54952))
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


fn listen(){
    let a:SocketAddr = match localip(){
        Ok(q)=>q,
        Err(e)=>{eprintln!("{}",e);return},
    };
    println!("binding to {}",a);
    globalip();
    let listener:TcpListener;
    match TcpListener::bind(a){
        Ok(q)=>listener=q,
        Err(e)=>{eprintln!("{}",e);return},
    }
    for s in listener.incoming(){
        match s{
            Ok(ss)=>connect(ss),
            Err(e)=>{eprintln!("{}",e);},
        }
    }
}

fn connect(mut stream: TcpStream){
    stream.write(&[69]);
}

pub fn gameloop() {
    listen(); // comment for the rendering and stuff or whatever
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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

        match gs.render() {
            Ok(_r) => {},
            Err(e) => eprintln!("{}", e),
        }


        let delta = begin.elapsed();

        let idle = 1_000_000_000u32 / FRAMERATE - delta.as_millis() as u32;

        std::thread::sleep(Duration::new(0, idle));
    }
}
