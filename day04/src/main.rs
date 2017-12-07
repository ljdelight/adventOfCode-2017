use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().take_while(|line| line.is_ok())
        .map(|line_result| line_result.unwrap())
        .collect();

    println!("Problem 1: count is {}", get_valid_passphrase_count(&lines));
    println!("Problem 2: count is {}", get_valid_passphrase_count_anagram(&lines));
}

fn get_valid_passphrase_count(phrases: &Vec<String>) -> usize {
    phrases.iter()
        .filter(|phrase| is_valid_passphrase(phrase))
        .count()
}

fn is_valid_passphrase(phrase: &String) -> bool {
    let mut words = HashSet::new();
    for word in phrase.split(" ") {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    true
}



fn get_valid_passphrase_count_anagram(phrases: &Vec<String>) -> usize {
    phrases.iter()
        .filter(|phrase| is_valid_passphrase_anagram(phrase))
        .count()
}

fn is_valid_passphrase_anagram(phrase: &String) -> bool {
    let mut words = HashSet::new();
    for word in phrase.split(" ") {
        let mut sorted = String::from(word).into_bytes();
        sorted.sort();
        let hash = String::from_utf8(sorted).unwrap();
        if words.contains(&hash) {
            return false;
        }
        words.insert(hash);
    }
    true
}
