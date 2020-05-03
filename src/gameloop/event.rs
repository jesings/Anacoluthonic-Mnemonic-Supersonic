use sdl2::event::Event;
use super::*;
use self::gamestate::*;

impl GameState {
    pub fn handle_events(&mut self) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit {..} => return false,
                Event::KeyDown {keycode, ..} => {keycode;},
                _ => {},
            }
        }
        true
    }
}
