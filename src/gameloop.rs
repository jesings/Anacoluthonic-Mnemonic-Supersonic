use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod event;
mod render;
#[path = "grid.rs"] mod grid;

pub fn gameloop() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Anacoluthonic Mnemonic Supersonic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
       let begin = Instant::now();
        canvas.clear();
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
        render::render(&mut canvas);

        canvas.present();
        let framerate = 4;

        let delta = begin.elapsed();

        let idle = 1_000_000_000u32 / framerate - delta.as_millis() as u32;

        std::thread::sleep(Duration::new(0, idle));
    }
}
