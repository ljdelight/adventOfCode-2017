use std::collections::HashSet;

fn main() {
    let test_solution = problem1(&vec![0, 2, 7, 0]);
    assert!(test_solution == (5, vec![2, 4, 1, 2]));
    assert!(problem2(&test_solution.1) == 4);


    let input = vec![5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6];

    let part1 = problem1(&input);
    println!("Problem 1 is {} cycles", part1.0);

    let part2 = problem2(&part1.1);
    println!("Problem 2 is {} cycles", part2);
}

fn problem2(memory: &Vec<i32>) -> i32 {
    problem1(memory).0
}

fn problem1(memory: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut cycles = 0;
    let mut memory = memory.clone();
    let mut memory_seen = HashSet::new();

    loop {
        // println!("Inspecting {}", memoryStr);
        if memory_seen.contains(&memory) {
            println!("Found a match {}", int_vec_to_string(&memory));
            break;
        }
        cycles += 1;
        memory_seen.insert(memory.clone());
        let max_index = get_index_of_max(&memory);
        redistribute(&mut memory, max_index);
    }
    (cycles, memory)
}

fn redistribute(memory: &mut Vec<i32>, index: usize) {
    let count = memory[index] as usize;
    memory[index] = 0;
    for i in 1..count+1 {
        let offset: usize = (index + i) % memory.len();
        memory[offset] += 1;
    }
    // println!("Redistributed to {}", int_vec_to_string(&memory));
}

fn get_index_of_max(memory: &Vec<i32>) -> usize {
    let mut max = memory[0];
    let mut max_idx = 0;
    for (idx, &val) in memory.iter().enumerate() {
        if val > max {
            max = val;
            max_idx = idx;
        }
    }
    max_idx
}

fn int_vec_to_string(memory: &Vec<i32>) -> String {
    let s: Vec<String> = memory.iter()
        .map(|it| it.to_string())
        .collect();
    s.join("")
}
