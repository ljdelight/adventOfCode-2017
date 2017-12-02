use std::io;

fn main() {
    assert!(part1("1122") == 3);
    assert!(part1("1111") == 4);
    assert!(part1("1234") == 0);
    assert!(part1("91212129") == 9);

    assert!(part2("1212") == 6);
    assert!(part2("1221") == 0);
    assert!(part2("123425") == 4);
    assert!(part2("123123") == 12);
    assert!(part2("12131415") == 4);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read input",
    );

    let input = input.trim();
    println!("Solution part 1: {}", part1(&input));
    println!("Solution part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum: u32 = 0;

    // Check if the first and last item are a match
    if input[0] == input[input.len() - 1] {
        sum += (input[0] - ('0' as u8)) as u32;
    }

    // Check if i == i-1 where i in [1,len) since the 0 and len-1 items were already compared
    let mut prev = &input[0];
    for item in &input[1..] {
        if item == prev {
            sum += (item - ('0' as u8)) as u32;
        }
        prev = item;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let offset: usize = input.len() / 2;
    let mut sum: u32 = 0;

    for (idx, &item) in input.iter().enumerate() {
        if item == input[(idx + offset) % input.len()] {
            sum += (item - ('0' as u8)) as u32;
        }
    }
    sum
}
