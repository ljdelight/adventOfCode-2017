use std::io;
use std::io::BufRead;

#[derive(Clone,Debug)]
struct FirewallLayer {
    location: i32,
    direction: i32,
    depth: i32,
    range: i32,
}

impl FirewallLayer {
    fn tick(&mut self) {
        // If we're about to fall off of either end, reverse the direction
        if self.location == 0 && self.direction < 0 || self.location + 1 == self.range && self.direction > 0 {
            self.direction *= -1;
        }
        self.location += self.direction;
    }

    fn severity(&self) -> i64 {
        self.depth as i64 * self.range as i64
    }
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

    let mut pairs = Vec::new();
    for line in lines {
        let s: Vec<&str> = line.split(": ").collect();
        let a = s[0].parse::<i32>().unwrap();
        let b = s[1].parse::<i32>().unwrap();
        pairs.push((a, b));
    }

    let max_depth = 1 + pairs.last().unwrap().0;
    let mut layers = Vec::new();
    for i in 0..max_depth {
        layers.push(FirewallLayer { location: 0, direction: 1, depth: i, range: 0});
    }

    for pair in pairs {
        layers[pair.0 as usize].depth = pair.0;
        layers[pair.0 as usize].range = pair.1;
    }

    let severity = p1(&mut layers.clone(), false);
    println!("Part 1: Severity is {}", severity.1);
}


fn p1(layers: &mut Vec<FirewallLayer>, stopWhenCaught: bool ) -> (bool, i64) {
    let mut severity = 0;
    let mut caught = false;
    for position in 0..layers.len() {
        if 0 == layers[position].location {
            // println!("CAUGHT at position {}", position);
            severity += layers[position].severity();
            caught = true;
            if stopWhenCaught {
                return (true, severity);
            }
        }

        for layer in &mut layers.iter_mut() {
            layer.tick();
        }
    }
    (caught, severity)
}
