use std::io;
use std::io::BufRead;

#[derive(Clone, Debug)]
struct Part {
    left: i64,
    right: i64,
}

fn rotate(part: &Part) -> Part {
    Part {
        left: part.right,
        right: part.left,
    }
}

fn solve(parts: Vec<Part>, bridge: &mut Vec<Part>, value: i64) -> i64 {
    let mut max_value = value;
    for (i, part) in parts.iter().enumerate() {
        // Only try if the part is connectable to the bridge
        // println!("Inspecting {:?} {:?}", bridge.last(), part);
        let compatible: Option<Part> = match bridge.last() {
            Some(component) => {
                if component.right == part.left {
                    Some(part.clone())
                } else if component.right == part.right {
                    Some(rotate(&part))
                } else {
                    None
                }
            }
            None => {
                if part.left == 0 {
                    Some(part.clone())
                } else if part.right == 0 {
                    Some(rotate(&part))
                } else {
                    None
                }
            }
        };

        if compatible.is_none() {
            // println!("Not compatible");
            continue;
        }

        let compat_part = compatible.unwrap();
        // println!("{:?}  ++ {:?} ", bridge, compat_part);
        let mut temp = parts.clone();
        temp.remove(i);
        bridge.push(compat_part.clone());
        let weight = solve(temp, bridge, value + compat_part.left + compat_part.right);
        if (weight > max_value) {
            // Print this valid bridge configuration; the output can be parsed to find
            // the longest bridge and compare weights.
            println!("{} {}", bridge.len(), weight);
            max_value = weight;
        }
        bridge.pop();
    }
    max_value
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| it.len() > 0)
        .collect();

    let mut parts = Vec::new();
    for line in lines {
        let s: Vec<&str> = line.split('/').collect();
        let a = s[0].parse::<i64>().unwrap();
        let b = s[1].parse::<i64>().unwrap();
        parts.push(Part { left: a, right: b });
    }

    let mut bridge = Vec::new();
    let max = solve(parts, &mut bridge, 0);
    println!("Strongest bridge is {}", max);
}
