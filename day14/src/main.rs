use std::u8;

fn main() {
    // tests();

    let input = "hxtvlmkl";
    // let input = "flqrgnkx"; // the test input

    println!(
        "Running to_grid({}) and computing knox bits. Found {} bits set",
        input,
        count_knot_bits(&input)
    );

    let matrix = to_matrix_int(&to_matrix_str(&input));
    let count = count_components(&matrix);
    println!("Found {} components", count);
}

fn count_components(matrix: &Vec<Vec<i32>>) -> i64 {
    let mut matrix = matrix.clone();
    let mut component_count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if connect_components(&mut matrix, -1 * (1 + component_count) as i32, (
                i as i32,
                j as i32,
            ))
            {
                component_count += 1;
            }
        }
    }

    component_count
}

fn connect_components(matrix: &mut Vec<Vec<i32>>, id: i32, position: (i32, i32)) -> bool {

    if matrix[position.0 as usize][position.1 as usize] <= 0 {
        return false;
    }
    let len = matrix.len() as i32;
    let mut queue = Vec::new();
    queue.push(position);
    while !queue.is_empty() {
        let mut node = queue.pop().unwrap();
        if !(node.0 >= 0 && node.0 < len) || !(node.1 >= 0 && node.1 < len) {
            continue;
        }
        if matrix[node.0 as usize][node.1 as usize] <= 0 {
            continue;
        }
        matrix[node.0 as usize][node.1 as usize] = id;

        let up = (node.0 - 1, node.1);
        // println!("At {:?} and Up is {:?}", node, up);
        queue.push(up);

        let down = (node.0 + 1, node.1);
        queue.push(down);

        let left = (node.0, node.1 - 1);
        queue.push(left);

        let right = (node.0, node.1 + 1);
        queue.push(right);
    }

    return true;
}


// p1
fn count_knot_bits(input: &str) -> u32 {
    let mut total = 0u32;
    let matrix = to_matrix_str(&input);
    for string in matrix {
        // println!("{}", string);
        total += string.chars().fold(0, |acc, it| if it == '1' {
            acc + 1
        } else {
            acc
        });
    }

    total
}

fn to_matrix_int(matrix_str: &Vec<String>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for (i, row) in matrix_str.iter().enumerate() {
        ret.push(Vec::new());
        for (j, byte) in row.bytes().enumerate() {
            ret[i].push((byte - b'0') as i32);
        }
    }
    ret
}

fn to_matrix_str(input: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for i in 0..128 {
        ret.push(String::new());
        let row = format!("{}-{}", input, i.to_string());
        let hash = knot_hash(&row);
        for (idx, byte) in hash.bytes().enumerate() {
            let asByte: u8 = match byte {
                b'a'...b'z' => byte - b'a' + 10,
                b'0'...b'9' => byte - b'0',
                _ => 0,
            };
            ret[i].push_str(&format!("{:04b}", asByte));
        }
    }
    ret
}


fn tests() {
    assert!(count_knot_bits("flqrgnkx") == 8108);
}


//
// Below is copied from my AoC day 10 solution.
//

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
