use super::*;
use super::gamestate::*;
use super::grid::Tile;
use super::entities::*;
use sdl2::rect::*;

static TILEDIM: u32 = 20;
static ITILEDIM: i32 = TILEDIM as i32;
static DTILEDIM: f64 = TILEDIM as f64;

impl GameState {
    pub fn render(&mut self) -> Result<bool, String> {
        self.set_draw_color(0, 0, 0);
        self.clear();


        let pp = self.player.pos();

        //draw tiles

        let dims = self.canvas.output_size()?;
        let xtiles: i32 = ((dims.0 + TILEDIM - 1) / TILEDIM) as i32; //hacky round up division
        let ytiles: i32 = ((dims.1 + TILEDIM - 1) / TILEDIM) as i32; //hacky round up division
        let startx = ((pp.x as f32) - (xtiles as f32) / 2.0).floor() as i32; //x11 bad, wayland good
        let starty = ((pp.y as f32) - (ytiles as f32) / 2.0).floor() as i32;
        let mut xcoord = -(DTILEDIM * pp.x.fract()) as i32;
        let ysto = -(DTILEDIM * pp.y.fract()) as i32;
        for i in 0..=xtiles {
            let mut ycoord = ysto;
            for j in 0..=ytiles {
                let tile: &Tile;
                let color: u8;
                match self.grid.grid_coord((startx + i) as usize, (starty + j) as usize) {
                    None => {
                        color = 0;
                    },
                    Some(t) => {
                        tile = t;
                        color = t.texture;
                    },
                }
                self.set_draw_color(color, color, color);
                let r = Rect::new(xcoord, ycoord, TILEDIM, TILEDIM);
                self.canvas.fill_rect(r)?;
                ycoord += ITILEDIM;
            }
            xcoord += ITILEDIM;
        }

        //TODO: draw entities

        //draw player
        self.set_draw_color(255, 0, 0);
        let hr = self.player.hitbox;
        let xlen = ((hr.x1 + hr.x2) * DTILEDIM) as u32;
        let ylen = ((hr.y1 + hr.y2) * DTILEDIM) as u32;
        let topx = (dims.0 / 2) as i32 - (hr.x1 * DTILEDIM) as i32;
        let topy = (dims.1 / 2) as i32 - (hr.y1 * DTILEDIM) as i32;
        let plyr = Rect::new(topx, topy, xlen, ylen);
        self.canvas.fill_rect(plyr)?;
        
        self.present();
        Ok(true)
    }
}
