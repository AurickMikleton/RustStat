use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

fn preprocess_banlist(file_path: &str) -> HashSet<String> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut words: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line = line.expect("failed to read");
        for word in line.split_whitespace().map(|w| w.to_lowercase()) { // I don't want to add
                                                                        // split by commas
            words.insert(word);
        }
    }
    return words;
}

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

    return words;
}

fn check_banned(banned: &HashSet<String>, word: &str) -> bool {
    banned.contains(word)
}

fn sort_words(banned: &HashSet<String>, words: &HashMap<String, usize>) {
    let mut hash_vector: Vec<(&String, &usize)> = words.iter().collect();
    hash_vector.sort_by(|a, b| b.1.cmp(a.1));

    let mut i: usize = 0;
    for (word, instances) in hash_vector {
        if check_banned(banned, word) {continue};
        if i >= 5 {break};
        println!("{} / {}", word, instances);
        i+=1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <file_path>");
        return;
    }
    let words = parse_words(&args[1]);
    let banned = preprocess_banlist(&args[2]);
    sort_words(&banned, &words);
}
