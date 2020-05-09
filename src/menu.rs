use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::ttf::Font;

trait MenuRender {
    fn render(&self, canv: &mut WindowCanvas, xdim: i32, ydim: i32) -> bool;
}

pub struct Button<'ttf, 'r> {
    height: f32,
    width: f32,
    cx: f32,
    cy: f32,
    text: String,
    //texture
    font: Font<'ttf, 'r>,
    textcolor: Color,
}

pub struct Slider {
    height: f32,
    width: f32,
    cx: f32,
    cy: f32,
    //nubtexture
    //linetexture
    nubheight: f32,//minimum of 1 px I should hope
    lineheight: f32, //minimum of 1 px I should hope
    nubpos: f32,
}

impl MenuRender for Button<'_, '_> {
    fn render(&self, canv: &mut WindowCanvas, xdim: i32, ydim: i32) -> bool {
        let iwidth = (self.width * xdim as f32) as i32;
        let iheight = (self.height * ydim as f32) as i32;
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - iwidth / 2;
        let corny = icy - iheight / 2;
        let wrecked = Rect::new(cornx, corny, iwidth as u32, iheight as u32);
        canv.set_draw_color(Color::RGB(0, 80, 160));
        match canv.draw_rect(wrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering button background, {}", e);
                return false;
            },
        }
        let partial = self.font.render(self.text.as_str());
        let mut textsurf = match partial.blended(self.textcolor) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error rendering text on button, {}", e);
                return false;
            },
        };

        let twidth = textsurf.width();
        let theight = textsurf.height();

        println!("{} {}", twidth, theight);

        //let texture_creator = canv.texture_creator();
        //let text = texture_creator.create_texture_from_surface(&mut textsurf).unwrap();
        //match self.canvas.copy_ex(&text, None, Rect::new(, topy, xlen, ylen), gdata.player.rot(), ppt, false, false) {
        
        true
    }
}
