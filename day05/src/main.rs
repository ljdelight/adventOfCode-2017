use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let maze: Vec<i32> = stdin.lock().lines().take_while(|line| line.is_ok())
        .map(|line_result| line_result.unwrap().parse().unwrap())
        .collect();
    
    println!("Part 1 Steps to exit {}", steps_to_exit(&maze));
    println!("Part 2 Steps to exit {}", steps_to_exit_p2(&maze));
}

fn steps_to_exit(maze: &Vec<i32>) -> i32 {
    let mut steps = 0i32;
    let mut idx = 0i32;
    let mut maze = maze.clone();
    while (true) {
        if idx < 0 || idx >= maze.len() as i32 {
            println!("Stopping at idx={}", idx);
            break;
        }
        let tmp = idx;
        let offset = maze[idx as usize];
        idx = idx + offset;
        maze[tmp as usize] += 1;
        steps += 1;
    }

    steps
}


fn steps_to_exit_p2(maze: &Vec<i32>) -> i32 {
    let mut steps = 0i32;
    let mut idx = 0i32;
    let mut maze = maze.clone();
    while (true) {
        if idx < 0 || idx >= maze.len() as i32 {
            println!("Stopping at idx={}", idx);
            break;
        }
        let tmp = idx;
        let offset = maze[idx as usize];
        idx = idx + offset;
        maze[tmp as usize] += if offset >= 3 { -1 } else { 1 };
        steps += 1;
    }

    steps
}
