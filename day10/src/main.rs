

fn vec_seq(len: i32) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in 0..len {
        ret.push(i);
    }
    ret
}

fn reverse_span(seq: &mut Vec<i32>, start_idx: i32, length: i32) {
    for offset in 0..(length + 1) / 2 {
        let src_idx: usize = ((start_idx + offset) % seq.len() as i32) as usize;
        let dst_idx: usize = ((start_idx + (length - 1) - offset) % seq.len() as i32) as usize;
        // println!("Swapping {} {}", src_idx, dst_idx);
        let temp = seq[src_idx];
        seq[src_idx] = seq[dst_idx];
        seq[dst_idx] = temp;
    }
}

fn do_round(seq: &mut Vec<i32>, length_list: &Vec<i32>, offset: &mut i32, skip_size: &mut i32) {
    for length in length_list {
        reverse_span(seq, *offset, *length);
        *offset += ((*length + *skip_size) % seq.len() as i32);
        *skip_size += 1;
    }
}

fn str_expand_to_binary(seq: &str) -> Vec<i32> {
    seq.bytes().map(|it| it as u8).map(|it| it as i32).collect()
}

fn main() {
    tests();
    let input = "187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216";
    println!("Solution for part 1: {}", part1(&input));
    println!("Solution for part 2: {}", knot_hash(&input));
}

fn part1(input: &str) -> i32 {
    let length_list: Vec<i32> = input.split(',').map(|it| it.parse().unwrap()).collect();
    let mut seq = vec_seq(256);
    let mut offset = 0i32;
    let mut skip_size = 0;
    // println!("Original {:?}", seq);
    do_round(&mut seq, &length_list, &mut offset, &mut skip_size);
    seq[0] * seq[1]
}

fn dense_hash(array: &Vec<i32>) -> u8 {
    let mut xor = 0;
    for val in array {
        xor ^= *val as u8;
    }
    xor
}

fn knot_hash(input: &str) -> String {
    let mut input = str_expand_to_binary(input);
    input.extend(vec![17, 31, 73, 47, 23]);

    let input = input;
    let mut seq = vec_seq(256);
    let mut offset = 0i32;
    let mut skip_size = 0;
    // println!("Original {:?}", seq);
    for round in 0..64 {
        do_round(&mut seq, &input, &mut offset, &mut skip_size);
    }
    let len = seq.len() / 16;
    let mut res = String::from("");
    for subset in 0..len {
        let slice = &seq[0 + 16 * subset..16 * (subset + 1)];
        let s = dense_hash(&slice.to_vec());
        let c = format!("{:02x}", s);
        res.push_str(&c);
    }
    res
}

fn tests() {
    //   (0 1 2 3 4)
    // -> 4 3 2 1 0
    let expected = vec![4, 3, 2, 1, 0];
    let mut res = vec_seq(5);
    reverse_span(&mut res, 0, 5);
    assert!(expected == res);

    //    0) (1 2 3 4
    // -> 1   0 4 3 2
    let expected = vec![1, 0, 4, 3, 2];
    let mut res = vec_seq(5);
    reverse_span(&mut res, 1, 5);
    assert!(expected == res);


    //    0 (1 2 3) 4
    // -> 0  3 2 1  4
    let expected = vec![0, 3, 2, 1, 4];
    let mut res = vec_seq(5);
    reverse_span(&mut res, 1, 3);
    assert!(expected == res);

    //    0 1 2) (3 4
    // -> 0 4 3   2 1
    let expected = vec![0, 4, 3, 2, 1];
    let mut res = vec_seq(5);
    reverse_span(&mut res, 3, 5);
    assert!(expected == res);


    let length_list = vec![3, 4, 1, 5];
    let mut seq = vec_seq(5);
    let mut offset = 0;
    let mut skip_size = 0;
    for length in length_list {
        reverse_span(&mut seq, offset, length);
        offset += (length + skip_size) % seq.len() as i32;
        skip_size += 1;
    }
    assert!(seq[0] * seq[1] == 12);

    // Part 2 tests
    assert!(str_expand_to_binary("1,2,3") == vec![49, 44, 50, 44, 51]);
    assert!(dense_hash(&vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]) == 64u8);

    assert!(knot_hash("") == "a2582a3a0e66e6e86e3812dcb672a272");
    assert!(knot_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd");
    assert!(knot_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d");
    assert!(knot_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e");
}
