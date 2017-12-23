use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn rotate(direction: Direction, node_state: &NodeState) -> Direction {
    match node_state {
        // If infected, look to the RIGHT.
        &NodeState::Infected => {
            match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        }
        // If clean, look to the LEFT.
        &NodeState::Clean => {
            match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }
        }
        // If flagged, reverse the direction.
        &NodeState::Flagged => {
            match direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        },
        // If weakened, look continue in the same direction
        &NodeState::Weakened => direction,
    }
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| it.len() > 0)
        .collect();
    lines
}

fn main() {
    let mut matrix = [[NodeState::Clean; 1024]; 1024];

    let input = get_input();
    for (i, line) in input.iter().enumerate() {
        for (j, &c) in line.as_bytes().iter().enumerate() {
            match c {
                b'.' => {}
                b'#' => {
                    matrix[i][j] = NodeState::Infected.clone();
                }
                _ => {
                    println!("INVALID INPUT");
                }
            }
        }
    }

    let mut i: usize = input[0].len() / 2;
    let mut j: usize = i;
    let mut direction = Direction::Up;
    let mut infections_caused = 0;

    for burst in 0..10000000 {

        // Rotate based on infection
        // let state: NodeState = if matrix[i][j] {NodeState::Infected} else {NodeState::Clean};
        // let state = matrix[i][j];
        direction = rotate(direction, &matrix[i][j]);

        // Transition the infection state at the current location
        matrix[i][j] = match &matrix[i][j] {
            &NodeState::Infected => NodeState::Flagged,
            &NodeState::Clean => NodeState::Weakened,
            &NodeState::Weakened => {
                infections_caused += 1;
                NodeState::Infected
            }
            &NodeState::Flagged => NodeState::Clean,
        };

        // Move forward one cell (take care not to fall off the side of the matrix)
        match direction {
            Direction::Up => {
                i = (((i as i64 - 1) + 1024) % 1024) as usize;
            }
            Direction::Down => {
                i = (i + 1) % 1024;
            }
            Direction::Left => {
                j = (((j as i64 - 1) + 1024) % 1024) as usize;
            }
            Direction::Right => {
                j = (j + 1) % 1024;
            }
        }
    }

    println!("Infected {} nodes", infections_caused);
}
