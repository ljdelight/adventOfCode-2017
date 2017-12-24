use std::io;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = get_input();
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        matrix.push(Vec::new());
        for &c in line.as_bytes().iter() {
            matrix[i].push(c);
        }
    }

    let mut i = 0;
    let mut j = matrix[0].iter().position(|&b| b == b'|').unwrap();
    let mut direction = Direction::Down;
    let mut visited_chars: Vec<char> = Vec::new();
    let mut total_steps = 1;
    loop {
        // consume
        match matrix[i][j] {
            b'a'...b'z' | b'A'...b'Z' => {
                visited_chars.push(matrix[i][j] as char);
            }
            _ => {}
        }

        // Move based on the direction
        let tuple = move_by_direction(&matrix, &direction, i as i64, j as i64);
        if tuple.0 <= 0 {
            println!(
                "Ran out of moves in {} steps. Visited {}",
                total_steps,
                visited_chars.into_iter().collect::<String>()
            );
            break;
        }
        total_steps += 1;
        i = tuple.0 as usize;
        j = tuple.1 as usize;
        direction = tuple.2;
    }
}


fn move_by_direction(
    matrix: &Vec<Vec<u8>>,
    direction: &Direction,
    i: i64,
    j: i64,
) -> (i64, i64, Direction) {
    println!("At {},{} facing {:?}", i, j, direction);
    let order = try_order(direction);
    for dir in &order {
        if is_okay_move(matrix, &dir, i, j) {
            let mut i = i;
            let mut j = j;
            match dir {
                &Direction::Up => i = i - 1,
                &Direction::Down => i = i + 1,
                &Direction::Left => j = j - 1,
                &Direction::Right => j = j + 1,
            }
            return (i, j, dir.clone());
        }
    }

    println!(
        "FAILED to find a next move. At {},{} facing {:?}",
        i,
        j,
        direction
    );
    (-1, -1, Direction::Down)
}

fn try_order(direction: &Direction) -> Vec<Direction> {
    match direction {
        &Direction::Up => vec![Direction::Up, Direction::Left, Direction::Right],
        &Direction::Down => vec![Direction::Down, Direction::Left, Direction::Right],
        &Direction::Left => vec![Direction::Left, Direction::Up, Direction::Down],
        &Direction::Right => vec![Direction::Right, Direction::Up, Direction::Down],
    }
}

fn is_okay_move(matrix: &Vec<Vec<u8>>, direction: &Direction, i: i64, j: i64) -> bool {
    let max_row_len = matrix.len() as i64;
    let max_col_len = matrix[0].len() as i64;
    let mut i = i;
    let mut j = j;
    match direction {
        &Direction::Up => i = i - 1,
        &Direction::Down => i = i + 1,
        &Direction::Left => j = j - 1,
        &Direction::Right => j = j + 1,
    }
    // Okay move if we're in bounds and the next character isn't an empty space
    i >= 0 && i < max_row_len && j >= 0 && j < max_col_len && matrix[i as usize][j as usize] != b' '
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
