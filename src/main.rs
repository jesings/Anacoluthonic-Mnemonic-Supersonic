use std::env;
mod gameloop;
mod client;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	if(args.len()==1){
		gameloop::gameloop();
	}else{
		client::connect(args.remove(1));
	}
    //sanity checks
}
