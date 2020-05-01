use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

pub struct Tile {
    texture: u8, //Not sure texture should be a u8, we can make it an SDL object later
}

pub struct Grid {
    cols: usize,
    rows: usize,
    tiles: Vec<Tile>,
}

impl Grid{
    pub fn new<T>(mapbytes: T, source: &str) -> Result<Grid, std::io::Error>
    where
        T: IntoIterator<Item = u8> {
        let mut vec: Vec<Tile> = Vec::new();
        
        let mut height = 0;
        let mut width = 0;
        let mut maxwidth = 0;

        for byte in mapbytes {
            if byte == 255 {
                if height == 0 {
                    maxwidth = width;
                } else {
                    if width != maxwidth {
                        eprintln!("Error: {} has more tiles on row {} than on row 0", source, height);
                        return Err(Error::new(ErrorKind::Other, "Map is malformed"));
                    }
                }
                width = 0;
                height += 1;
            } else {
                vec.push(Tile{texture: byte});
                width += 1;
            }
        }
        if width != 0 || height == 0 {
            eprintln!("Error: {} does not end in a 0xff endrow character", source);
            return Err(Error::new(ErrorKind::Other, "Map is malformed"));
        }

        Ok(Grid{
            cols: maxwidth,
            rows: height,
            tiles: vec,
        })
    }
    pub fn new_from_file(mapfile: &str) -> Result<Grid, std::io::Error> {

        let mut file = match File::open(mapfile) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Specified Mapfile {} does not exist!", mapfile);
                return Err(e);
            },
        };
        let mut bytevec: Vec<u8> = Vec::new();
        match file.read_to_end(&mut bytevec) {
            Ok(_f) => {
                Grid::new(bytevec, mapfile)
            },
            Err(e) => {
                eprintln!("Error: Error in reading Mapfile {}!", mapfile);
                return Err(e)
            },
        }
    }
    pub fn grid_coord(&self, r: usize, c: usize) -> Option<&Tile>{
        if r > self.rows || c > self.cols {
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
