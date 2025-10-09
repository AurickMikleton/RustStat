use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn parse_words(file_path: &str)  -> HashMap<String, usize> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut words: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("failed to read");
        for word in line.split_whitespace().map(|w| w.to_lowercase()) {
            *words.entry(word).or_insert(0) += 1;
        }
    }

    let mut hash_vec: Vec<(&String, &usize)> = words.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, instances) in hash_vec {
        println!("{} / {}", word, instances);
    }

    return words;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <file_path>");
        return;
    }
    let _ = parse_words(&args[1]);
}
