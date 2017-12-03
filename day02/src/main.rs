use std::io;
use std::fs::File;

use std::io::{BufRead, BufReader};


fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().take_while(|line| line.is_ok())
        .map(|line_result| line_result.unwrap())
        .collect();

    let prob1 = lines.iter()
        .map(|line| get_checksum_of_row(line))
        .fold(0i64, |acc, it| acc + it);
    println!("Solution 1: {}", prob1);

    let prob2 = lines.iter()
        .map(|line| get_divisible_result_of_row(line))
        .fold(0i64, |acc, it| acc + it);
    println!("Solution 2: {}", prob2);
}

fn get_checksum_of_row(row: &str) -> i64 {
    let arr: Vec<i64> = row.split(char::is_whitespace)
        .map(|n| n.parse().unwrap())
        .collect();
    let mut min = &arr[0];
    let mut max = &arr[0];
    for v in &arr {
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
    }
    max - min
}


fn get_divisible_result_of_row(row: &str) -> i64 {
    let arr: Vec<i64> = row.split(char::is_whitespace)
        .map(|n| n.parse().unwrap())
        .collect();
    for (i, iItem) in arr.iter().enumerate() {
        for (j, jItem) in arr[i+1..].iter().enumerate() {
            let larger = if iItem > jItem {iItem} else {jItem};
            let smaller = if iItem > jItem {jItem} else {iItem};
            if larger % smaller == 0 {
                println!("Divisible: {} {} = {}", larger, smaller, larger / smaller);
                return larger / smaller;
            }
        }
    }

    println!("ERROR no divisible pairs found in {}", row);
    -1
}
