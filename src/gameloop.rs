use std::time::{Duration, Instant};

#[path = "grid.rs"] mod grid;
#[path = "entities.rs"] mod entities;
mod gamestate;
mod render;
mod event;

pub fn gameloop() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Anacoluthonic Mnemonic Supersonic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut grid: grid::Grid;
    match grid::Grid::random_grid(400, 400){
        Ok(g) => grid = g,
        Err(_e) => return,
    }

    let mut gs = gamestate::GameState{
        canvas: canvas,
        grid: grid,
        player: entities::Player::new(),
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    let framerate = 4;

    'running: loop {
        let begin = Instant::now();
        match event::handle_events(&mut event_pump) {
            false => break 'running,
            true => {},
        }
        // The rest
        // of the
        // game
        // loop
        // goes
        // here...
        match render::render(&mut gs) {
            Ok(_r) => {},
            Err(e) => eprintln!("{}", e),
        }


        let delta = begin.elapsed();

        let idle = 1_000_000_000u32 / framerate - delta.as_millis() as u32;

        std::thread::sleep(Duration::new(0, idle));
    }
}
