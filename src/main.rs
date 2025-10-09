use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn parse_words(file_path: &str)  -> Result<(String, usize), Box<dyn std::error::Error>> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut tree = BTreeMap::new();

    for line in reader.lines() {
        let line = line.expect("failed to read");
        for word in line.split_whitespace().map(|w| w.to_lowercase()) {
            *tree.entry(word).or_insert(0) += 1;
            //println!("{}", word);
        }
    }

    let max_entry = tree.into_iter().max_by_key(|(_, count)| *count);
    match max_entry {
        Some((word, count)) => Ok((word, count)),
        None => Err("No words".into()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <file_path>");
        return;
    }
    let word = parse_words(&args[1]).expect("");
    println!("{} / {}", word.0, word.1);
}
