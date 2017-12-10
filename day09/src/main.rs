use std::io;
use std::io::stdin;
use std::io::BufRead;

fn clean_stream(stream: &String) -> (String, i32) {
    let bytes = stream.as_bytes();
    let mut garbageRemovedCount = 0;
    let mut garbage = false;
    let mut cleanStream = String::new();

    let mut idx = 0;
    loop {
        if idx == bytes.len() {
            break;
        }

        match bytes[idx] as char {
            '<' if !garbage => {
                garbage = true;
            }
            '>' if garbage => {
                garbage = false;
            }
            '!' if garbage => {
                // Skip the next token
                idx += 1;
            }
            c if !garbage => {
                cleanStream.push(c);
            }
            _ => {
                // Skipping garbage
                garbageRemovedCount += 1;
            }
        }
        idx += 1;
    }
    (cleanStream, garbageRemovedCount)
}

fn score(input: &str) -> i32 {
    println!("Scoring '{}'", input);
    let bytes = input.as_bytes();
    let mut score = 0;
    let mut nest = 0;
    for c in bytes {
        match *c as char {
            '{' => {
                score += nest + 1;
                nest += 1;
            }
            '}' => {
                nest -= 1;
            }
            _ => {}
        }
    }

    score
}

fn main() {
    let testsP1 = vec![
        ("{}", 1),
        ("{{{}}}", 6),
        ("{{},{}}", 5),
        ("{{{},{},{{}}}}", 16),
        ("{<a>,<a>,<a>,<a>}", 1),
        ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
        ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
        ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
    ];
    for test in testsP1 {
        let res = clean_stream(&String::from(test.0));
        assert!(score(&res.0) == test.1);
    }
    let testsP2 = vec![
        ("<>", 0),
        ("<random characters>", 17),
        ("<<<<>", 3),
        ("<{!>}>", 2),
        ("<!!>", 0),
        ("<!!!>>", 0),
        ("<{o'i!a,<{i<a>", 10),
    ];
    for test in testsP2 {
        let res = clean_stream(&String::from(test.0));
        assert!(res.1 == test.1);
    }

    let stdin = io::stdin();
    let input: String = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|res| res.unwrap())
        .fold(String::new(), |mut acc, it| {
            acc.push_str(&*it);
            acc
        });

    // println!("Input is '{}'", input);
    let cleaned = clean_stream(&input);
    println!(
        "Cleaned input has score {} with garbageCount={}",
        score(&cleaned.0),
        cleaned.1
    );
}
