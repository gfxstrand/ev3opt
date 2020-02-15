use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} filename", args[0]);
        process::exit(1);
    }

    let query = &args[1];

    println!("{:?}", query);
}
