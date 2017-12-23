
fn walk(list: &mut Vec<i32>, pos: usize, steps: usize, value: i32) -> usize {
    let idx = (pos + steps) % list.len();
    list.insert(idx, value);
    idx + 1
}

fn part1(steps: usize) -> i32 {
    let mut list = vec![0];
    let mut idx = 0;
    for value in 1..2017 + 1 {
        idx = walk(&mut list, idx, steps, value);
    }
    list[idx]
}

fn part2(steps: usize) -> usize {
    let mut idx = 0;
    let mut solution = 0;
    for i in (1..50_000_001) {
        idx = (idx + steps) % i;
        if idx == 0 {
            solution = i;
        }
        idx += 1;
    }
    solution
}

fn main() {
    println!("FOUND {}", part1(3));
    assert!(part1(3) == 638);
    println!("Part 1: Finished value = {}", part1(345));
    println!(
        "Part 2: After 50_000_000 insertions, after '0' is {}",
        part2(345)
    );
}
