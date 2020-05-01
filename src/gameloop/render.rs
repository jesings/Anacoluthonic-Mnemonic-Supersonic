use super::gamestate::*;
use super::grid::*;

pub fn render(gs: &mut GameState) -> Result<bool, String> {
    gs.set_draw_color(0, 0, 0);
    gs.clear();
    for i in (0..gs.grid.rows).rev() {
        for j in 0..gs.grid.cols {
            let tile: &Tile;
            let color: u8;
            match gs.grid.grid_coord(i, j) {
                None => color = 0,
                Some(t) => {
                    tile = t;
                    color = t.texture
                },
            }
            gs.set_draw_color(color, color, color);
            gs.draw_point(i as i32, j as i32)?;
        }
    }
    gs.present();
    Ok(true)
}
