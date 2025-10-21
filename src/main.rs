use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

struct TokenizedData {
    words: HashMap<String, usize>,
    name: String,
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

fn count_scentences(line: &str) -> u32 {
    let scentence_punctuation = ['.', '?', '!', ';'];
    let mut count: u32 = 0;
    let mut i: u32 = 0;
    for _ in line.split(&scentence_punctuation[..]) {
        if i > 0 {count += 1;}
        i += 1
    }
    return count;
}

fn is_not_alphabetic(c: char) -> bool {
    !char::is_alphabetic(c)
}

fn parse_words(file_path_buffer: &std::path::PathBuf, name: &str) -> TokenizedData {
    let file = File::open(file_path_buffer).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut words: HashMap<String, usize> = HashMap::new();
    let mut scentence_count: u32 = 0;
    let mut word_count: u32 = 0;
    for line in reader.lines() {
        let line = line.expect("failed to read");
        for scentences in line.split(is_not_alphabetic) {
            for word in scentences.split_whitespace().map(|w| w.to_lowercase()) {
                *words.entry(word).or_insert(0) += 1;
                word_count += 1;
            }
        }
        scentence_count += count_scentences(&line);
    }
    return TokenizedData {
        words: words,
        scentence_count: scentence_count,
        word_count: word_count,
        name: name.to_string(),
    };
}

fn generate_output(banned: &HashSet<String>, data: &TokenizedData) {
    let mut hash_vector: Vec<(&String, &usize)> = data.words.iter().collect();
    hash_vector.sort_by(|a, b| b.1.cmp(a.1));
    print!("{} / ", data.name);
    let mut i: usize = 0;
    for (word, _) in hash_vector {
        if banned.contains(word) {continue}
        if i >= 5 {break}
        print!("{} / ", word);
        i+=1;
    }
    print!("{}\n", data.word_count/data.scentence_count);
}

fn bulk_statistics(dir_path: &str, banlist_path: &str) {
    let directory = fs::read_dir(dir_path).unwrap();
    for file in directory {
        let file = file.unwrap();
        let file_path = file.path();
        let file_name: String = file.file_name().into_string().unwrap();
        let words = parse_words(&file_path, &file_name);
        let banned = preprocess_banlist(banlist_path);
        generate_output(&banned, &words);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage <dir_path> <file_path>");
        return;
    }
    bulk_statistics(&args[1], &args[2]);
}
