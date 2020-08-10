#![allow(dead_code)]

use std::env;
use std::path::Path;

mod gameloop;
mod server;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let path = Path::new(&args[0]);
    let ancillary = path.parent().unwrap();
    env::set_current_dir(ancillary).expect("Ancillary is gonna die");
    let backtrack = Path::new("..");
    env::set_current_dir(backtrack).expect("I cannot go to parent directory because I have deep seated childhood trauma");
    env::set_current_dir(backtrack).expect("I cannot go to grandparent directory my parents have deep seated childhood trauma");
    
    if args.len()>1{
        gameloop::gameloop(args.remove(1));
    }else{
        server::host();
    }
    //sanity checks (twitter engineering be like ree)
}
