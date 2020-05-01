use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in (*event_pump).poll_iter() {
        match event {
            Event::Quit {..} => return false,
            _ => {},
        }
    }
    true
}
