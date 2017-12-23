use std::io;
use std::io::BufRead;
use std::collections::HashSet;

type Tuple3 = (i64, i64, i64);

fn sum(a: Tuple3, b: Tuple3) -> Tuple3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
struct AccelParticle {
    id: i64,
    p: Tuple3,
    v: Tuple3,
    a: Tuple3,
}

impl AccelParticle {
    fn update_position(&mut self) {
        self.v = sum(self.v, self.a);
        self.p = sum(self.p, self.v);
    }

    fn distance(&self) -> i64 {
        self.p.0.abs() + self.p.1.abs() + self.p.2.abs()
    }
}

fn part1(particles: &Vec<AccelParticle>) -> i64 {
    let mut closest_id = 0;
    let mut closest_len = particles[0].distance();
    for p in particles {
        if p.distance() < closest_len {
            closest_id = p.id;
            closest_len = p.distance();
        }
    }
    closest_id
}

fn part2(particles: &Vec<AccelParticle>) -> i64 {
    let mut particles = particles.clone();

    for i in 0..300 {
        particles.sort_unstable_by_key(|p| p.p);
        for i in 0..(particles.len() - 1) {
            if i >= particles.len() - 1 {
                break;
            }

            while particles[i].p == particles[i + 1].p {
                while (i < particles.len() - 1) && particles[i].p == particles[i + 1].p {
                    particles.remove(i + 1);
                }
                particles.remove(i);
            }
        }
        for p in &mut particles {
            p.update_position();
        }
    }
    particles.len() as i64
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

    let mut particles: Vec<AccelParticle> = Vec::new();
    for (id, line) in lines.iter().enumerate() {
        let tokens: Vec<i64> = line.split_whitespace()
            .map(|it| it.parse().unwrap())
            .collect();
        particles.push(AccelParticle {
            id: id as i64,
            p: (tokens[0], tokens[1], tokens[2]),
            v: (tokens[3], tokens[4], tokens[5]),
            a: (tokens[6], tokens[7], tokens[8]),
        });
    }

    println!("Part 1: {}", part1(&particles));
    println!("Part 2: {}", part2(&particles));
}
