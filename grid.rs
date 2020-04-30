use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

trait Droppable {
}
trait TileModifier {
}

pub struct Tile {
    texture: u8, //Not sure texture should be a u8, we can make it an SDL object later
}
pub struct Grid {
    cols: usize,
    rows: usize,
    tiles: Vec<Tile>,
}
impl Grid{
    pub fn new(mapfile: &str) -> Result<Grid, std::io::Error> {

        let file = match File::open(mapfile) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Specified Mapfile {} does not exist!", mapfile);
                return Err(e)
            },
        };
        let mut vec: Vec<Tile> = Vec::new();
        
        let mut height = 0;
        let mut width = 0;
        let mut maxwidth = 0;

        for byte in file.bytes(){
            match byte{
                Err(e) => {
                    eprintln!("Error: Encountered an invalid byte read in mapfile {}", mapfile);
                    return Err(e);
                }
                Ok(b) => {
                    if b == 255 {
                        if height == 0 {
                            maxwidth = width;
                        } else {
                            if width != maxwidth {
                                eprintln!("Error: Mapfile {} has more tiles on row {} than on row 0", mapfile, height);
                                return Err(Error::new(ErrorKind::Other, "Map file is malformed"));
                            }
                        }
                        width = 0;
                        height += 1;
                    } else {
                        vec.push(Tile{texture: b});
                        width += 1;
                    }
                }
            }
        }
        if width != 0 {
            eprintln!("Error: Mapfile {} does not end in a 0xff endrow character", mapfile);
            return Err(Error::new(ErrorKind::Other, "Map file is malformed"));
        }

        Ok(Grid{
            cols: maxwidth,
            rows: height,
            tiles: vec,
        })
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

fn main(){
    let grid: Grid;
    if let Ok(g) = Grid::new("test.map"){
        grid = g;
    } else {
        println!("Map file is malformed or does not exist.");
        return;
    }
    grid.print_grid();
}
