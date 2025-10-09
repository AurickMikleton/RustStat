use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_words(file_path: &str) {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("failed to read");
        for word in line.split_whitespace() {
            println!("{}", word);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <file_path>");
        return;
    }
    parse_words(&args[1]);
    //let contents = fs::read_to_string(args[1].clone()).expect("Failed to read file");
    //println!("{}", contents);
}
