use std::io;
use std::io::BufRead;

fn min_steps(steps: &Vec<&str>) -> (i32, i32) {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut z = 0i32;
    let mut max = 0;
    for &c in steps {
        match c {
            "n" => {
                y += 1;
                z -= 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            _ => {
                println!("UNEXPECTED INPUT {}", c);
            }
        }
        let dist = (x.abs() + y.abs() + z.abs()) / 2;
        if dist > max {
            max = dist;
        }
    }
    let min = (x.abs() + y.abs() + z.abs()) / 2;
    (min, max)
}

fn stdin_one_line() -> String {
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
    input
}

fn main() {
    tests();
    let input = stdin_one_line();
    let solution = min_steps(&input.split(",").collect());
    println!(
        "Min steps is {}. Maximum distance is {}",
        solution.0,
        solution.1
    );
}

fn tests() {
    assert!(min_steps(&"ne,ne,ne".split(",").collect()).0 == 3);
    assert!(min_steps(&"ne,ne,sw,sw".split(",").collect()).0 == 0);
    assert!(min_steps(&"ne,ne,s,s".split(",").collect()).0 == 2);
    assert!(min_steps(&"se,sw,se,sw,sw".split(",").collect()).0 == 3);
}
