use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

struct TokenizedData {
    words: HashMap,
    word_count: u32,
    scentence_count: u32,
}

fn preprocess_banlist(file_path: &str) -> HashSet<String> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut words: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line = line.expect("failed to read");
        for word in line.split(&['\n', ','][..]).map(|w| w.to_lowercase()) {
            words.insert(word);
        }
    }
    return words;
}

fn parse_words(file_path: &str)  -> HashMap<String, usize> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut words: HashMap<String, usize> = HashMap::new();
    let whitespace = ['\n', ' '];
    let punctuation = ['.', '"', '.', '?', ',', '!'];
    let mut scentence_count: u32 = 0;
    let mut word_count: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("failed to read");
        for scentences in line.split(&punctuation[..]) {
            for word in scentences.split(&whitespace[..]).map(|w| w.to_lowercase()) {
                *words.entry(word).or_insert(0) += 1;
                word_count += 1;
            }
            scentence_count += 1;
        }
    }

    return words;
}

fn sort_words(banned: &HashSet<String>, words: &HashMap<String, usize>) {
    let mut hash_vector: Vec<(&String, &usize)> = words.iter().collect();
    hash_vector.sort_by(|a, b| b.1.cmp(a.1));

    let mut i: usize = 0;
    for (word, instances) in hash_vector {
        if banned.contains(word) {continue};
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
