use super::gamestate::*;
use super::grid::Tile;
use super::entities::*;
use super::menu::*;
use sdl2::rect::*;

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::pixels::Color;

static TILEDIM: u32 = 20;
static ITILEDIM: i32 = TILEDIM as i32;
static DTILEDIM: f64 = TILEDIM as f64;

impl GameState<'_, '_> {
    pub fn render(&mut self) -> Result<bool, String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let dims = self.canvas.output_size()?;


        match &self.scene {
            Scenes::GamePlay(_) => {
                let gdata = self.gamedata.lock().unwrap();
                let pp = gdata.players[gdata.pid].pos();

                //draw tiles

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
                        match gdata.grid.as_ref().unwrap().grid_coord((startx + i) as usize, (starty + j) as usize) {
                            None => {
                                color = 0;
                            },
                            Some(t) => {
                                tile = t;
                                color = tile.texture;
                            },
                        }
                        self.canvas.set_draw_color(Color::RGB(color, color, color));
                        let r = Rect::new(xcoord, ycoord, TILEDIM, TILEDIM);
                        self.canvas.fill_rect(r)?;
                        ycoord += ITILEDIM;
                    }
                    xcoord += ITILEDIM;
                }

                //let screenrect = Rect::new(0, 0, dims.0, dims.1);
                //TODO: draw entities

                //draw player
                self.canvas.set_draw_color(Color::RGB(255, 0, 0));

                let pdim = gdata.players[gdata.pid].dims();
                let ppt = Point::new((pdim.x * DTILEDIM) as i32, (pdim.y * DTILEDIM) as i32);
                let xlen = ((pdim.x * 2.0) * DTILEDIM) as u32;
                let ylen = ((pdim.y * 2.0) * DTILEDIM) as u32;
                let topx = (dims.0 / 2) as i32 - (pdim.x * DTILEDIM) as i32;
                let topy = (dims.1 / 2) as i32 - (pdim.y * DTILEDIM) as i32;
                let texture_creator = self.canvas.texture_creator();
                let mut surf = Surface::new(xlen, ylen, PixelFormatEnum::RGB24).unwrap();
                let text = texture_creator.create_texture_from_surface(&mut surf).unwrap();
                match self.canvas.copy_ex(&text, None, Rect::new(topx, topy, xlen, ylen), gdata.players[gdata.pid].rot(), ppt, false, false) {
                    Ok(_f) => {},
                    Err(_e) => {eprintln!("error in rendering player");},
                }

                //draw other players
                for (i,pphead) in gdata.players.iter().enumerate(){
                    if i!=gdata.pid{
                        let opp = pphead.pos();
                        let optopx = ((opp.x-pp.x)*DTILEDIM) as i32 + topx;
                        let optopy = ((opp.y-pp.y)*DTILEDIM) as i32 + topy;
                        //todo add texture shit
                        self.canvas.copy_ex(&text, None, Rect::new(optopx, optopy, xlen, ylen), pphead.rot(), ppt, false, false).expect("Cannot render because has no spatial awareness like benson");
                    }
                }

                for huditem in &self.huditems {
                    huditem.render(&mut self.canvas, dims.0 as i32, dims.1 as i32);
                }
                
            },
            Scenes::Menu(m) => {
              self.canvas.clear();
              for button in &m.buttons {
                button.render(&mut self.canvas, &mut self.fonts, dims.0 as i32, dims.1 as i32);
              }
            },
        }
        match &self.console {
            Some(c) => {
                c.render_console(&mut self.canvas);
            },
            None => {},
        }
        self.canvas.present();
        Ok(true)
    }
}
