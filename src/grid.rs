use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::convert::TryInto;
use rand_core::RngCore;

static TILEDIM: i32 = 20;

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

    pub fn random_grid(width: usize, height: usize, seed:u128) -> Result<Grid, std::io::Error>{
        let mut ayn: rand_pcg::Pcg64Mcg = rand_pcg::Pcg64Mcg::new(seed);
        let mut vecmap : Vec<u8> = Vec::with_capacity(width * height);
        for _i in 0 .. width * height {
            vecmap.push((ayn.next_u32()&255) as u8);
        }
        Grid::new(vecmap, "random", width, height)
    }

    pub fn grid_coord(&self, r: usize, c: usize) -> Option<&Tile>{
        if r >= self.rows || c >= self.cols {
            None
        } else {
            Some(&self.tiles[r * self.cols + c])
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
        let x0 = x_orig * TILEDIM + x_subpixel_orig;
        let y0 = y_orig * TILEDIM + y_subpixel_orig;
        let x1 = x_dest * TILEDIM + x_subpixel_dest;
        let y1 = y_dest * TILEDIM + y_subpixel_dest;
        let x_subpixels = x1 - x0;
        let mut y_subpixels = y1 - y0;
        let yi = if y_subpixels < 0 {y_subpixels = -y_subpixels; -1} else {1};
        //to handle octant 8

        let mut difference = 2 * y_subpixels - x_subpixels;
        let mut ynow = y0;

        for x in x0..x1 {
            //skip logic in here
            // if impassible return false
            match self.grid_coord((x / TILEDIM) as usize, (ynow / TILEDIM) as usize) {
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
