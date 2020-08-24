use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::convert::TryInto;
use rand_core::RngCore;

pub static TILEDIM: u32 = 20;
pub static ITILEDIM: i32 = TILEDIM as i32;
pub static DTILEDIM: f64 = TILEDIM as f64;
static ROOMFRAC: f32 = 0.25;
static MAXROOMSIZE: usize = 30;
static MINROOMSIZE: usize = 12;
static ROOMRANGE: usize = MAXROOMSIZE - MINROOMSIZE;


macro_rules! maperror {
    ($x: expr, $y: expr, $($z: expr),*) => {
        eprintln!($y, $($z),*);
        return Err(Error::new(ErrorKind::Other, $x));
    };
}
macro_rules! readerror {
    ($x: expr, $y: expr, $($z: expr),*) => {
        eprintln!($y, $($z),*);
        return Err($x);
    };
}

pub struct Tile {
    pub texture: u8, //Not sure texture should be a u8, we can make it an SDL object later
    pub passable: bool,
}

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    tiles: Vec<Tile>,
}

impl Grid{
    pub fn new<T>(mapbytes: T, source: &str, width: usize, height: usize) -> Result<Grid, std::io::Error>
    where T: IntoIterator<Item = u8> {
        let mut vec: Vec<Tile> = Vec::new();
        
        let mut read = 0;

        for byte in mapbytes {
            vec.push(Tile{texture: byte, passable: true});
            read += 1;
        }

        if height != read / width || read % width != 0 {
            maperror!("Map is malformed", "Error: {} has {} tiles, {} expected", source, read, width * height);
        }

        Ok(Grid{
            cols: width,
            rows: height,
            tiles: vec,
        })
    }

    pub fn new_from_file(mapfile: &str) -> Result<Grid, std::io::Error> {
        let mut file = match File::open(mapfile) {
            Ok(f) => f,
            Err(e) => {readerror!(e, "Error: Specified Mapfile {} does not exist!", mapfile);},
        };
        let mut bytevec: Vec<u8> = Vec::new();
        let mut dims = [0 as u8; 16];
        match file.read_exact(&mut dims) {
            Ok(_f) => {},
            Err(e) => {readerror!(e, "Error: Error in reading Mapfile {}!", mapfile);},
        }
        match file.read_to_end(&mut bytevec) {
            Ok(_f) => {
                Grid::new(bytevec, mapfile, 
                          usize::from_ne_bytes(dims[0 .. 8].try_into().unwrap()),
                          usize::from_ne_bytes(dims[8 .. 16].try_into().unwrap()))
            },
            Err(e) => {readerror!(e, "Error: Error in reading Mapfile {}!", mapfile);},
        }
    }

    pub fn new_from_roomgen(width: usize, height: usize, seed:u128) -> Result<Grid, std::io::Error>{
        let mut genvec = vec![64u8; width * height];
        let mut paul: rand_pcg::Pcg64Mcg = rand_pcg::Pcg64Mcg::new(seed);
        #[derive(Debug)]
        struct Rect(usize, usize, usize, usize);
        let mut clear_room = |rect: &Rect| {
            for y in rect.1 .. rect.3 {
                let rowoffset = y * width;
                for x in rect.0 .. rect.2 {
                    genvec[rowoffset + x] = 255u8;
                }
            }
        };
        let rect_center = |rect: Rect| ((rect.0 + rect.2) / 2, (rect.1 + rect.3) / 2);
        let intersect_rect = |rect: &Rect, other: &Rect| (rect.0 <= other.2) && (rect.2 >= other.0) && (rect.1 <= other.3) && (rect.3 >= other.1);
        let mut rooms: Vec<Rect> = vec!();
        let mut tiles_uncovered = 0;
        let tiles_thresh = ((width * height) as f32 * ROOMFRAC) as usize;
        'roomloop: while tiles_uncovered < tiles_thresh {
            let w = (paul.next_u32() as usize % ROOMRANGE) + MINROOMSIZE;
            let h = (paul.next_u32() as usize % ROOMRANGE) + MINROOMSIZE;
            let x = paul.next_u32() as usize % (width - w - 1);
            let y = paul.next_u32() as usize % (height - h - 1);
            let roomrect = Rect(x, y, x + w, y + h);
            for otherroom in &rooms {
                if intersect_rect(&roomrect, otherroom) {
                    continue 'roomloop;
                }
            }
            clear_room(&roomrect);
            rooms.push(roomrect);
            tiles_uncovered += w * h
        }
        Grid::new(genvec, "roomgen", width, height)
    }

    pub fn random_grid(width: usize, height: usize, seed:u128) -> Result<Grid, std::io::Error>{
        let mut ayn: rand_pcg::Pcg64Mcg = rand_pcg::Pcg64Mcg::new(seed);
        let mut vecmap = vec![0u8;width * height];
        ayn.fill_bytes(vecmap.as_mut_slice());
        Grid::new(vecmap, "random", width, height)
    }

    pub fn grid_coord(&self, r: usize, c: usize) -> Option<&Tile>{
        if r >= self.rows || c >= self.cols {
            None
        } else {
            Some(&self.tiles[r * self.cols + c])
        }
    }
    pub fn mut_grid_coord(&mut self, r: usize, c: usize) -> Option<&mut Tile>{
        if r >= self.rows || c >= self.cols {
            None
        } else {
            Some(&mut self.tiles[r * self.cols + c])
        }
    }

    pub fn print_grid(&self){
        let mut rowpos = 0;
        for t in self.tiles.iter() {
            match t.texture {
                0 => print!("  "),
                1 => print!("[]"),
                2 => print!("--"),
                _ => print!("??"),
            }
            rowpos += 1;
            if rowpos == self.cols {
                rowpos = 0;
                println!();
            }
        }
    }

    //Force ordering for bresenham, get octant

    pub fn bresen(&self, x_orig: i32, y_orig: i32, x_dest: i32, y_dest: i32, x_subpixel_orig: i32, y_subpixel_orig: i32, x_subpixel_dest: i32, y_subpixel_dest: i32) -> bool{
        //octant 1, 8 case only
        let x0 = x_orig * ITILEDIM + x_subpixel_orig;
        let y0 = y_orig * ITILEDIM + y_subpixel_orig;
        let x1 = x_dest * ITILEDIM + x_subpixel_dest;
        let y1 = y_dest * ITILEDIM + y_subpixel_dest;
        let x_subpixels = x1 - x0;
        let mut y_subpixels = y1 - y0;
        let yi = if y_subpixels < 0 {y_subpixels = -y_subpixels; -1} else {1};
        //to handle octant 8

        let mut difference = 2 * y_subpixels - x_subpixels;
        let mut ynow = y0;

        for x in x0..x1 {
            //skip logic in here
            // if impassible return false
            match self.grid_coord((x / ITILEDIM) as usize, (ynow / ITILEDIM) as usize) {
                Some(tile) => {
                    if !tile.passable {
                        return false;
                    }
                },
                None => {},
            }
            if difference > 0 {
                ynow += yi;
                difference -= 2 * x_subpixels;
            }
            difference += 2 * y_subpixels;
        }

        true
    }
}

/* fn main(){
    let grid: Grid;
    if let Ok(g) = Grid::new_from_file("test.map"){
        grid = g;
    } else {
        println!("Map file is malformed or does not exist.");
        return;
    }
    grid.print_grid();
} */
