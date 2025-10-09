use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <file_path>");
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).expect("Failed to read file");
    println!("{}", contents);
}
