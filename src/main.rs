use std::env;
use std::path::Path;

mod gameloop;
mod client;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    let path = Path::new(&args[0]);
    let ancillary = path.parent().unwrap();
    env::set_current_dir(ancillary);
    let backtrack = Path::new("..");
    env::set_current_dir(backtrack);
    env::set_current_dir(backtrack);
    
    if args.len()==1 {
        gameloop::gameloop();
    }else{
        client::connect(args.remove(1));
    }
    //sanity checks
}
