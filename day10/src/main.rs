

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

fn main() {
    tests();
    let input = "187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216";
    let length_list = input.split(',').map(|it| it.parse().unwrap());
    let mut seq = vec_seq(256);
    let mut offset = 0i32;
    let mut skipSize = 0;
    println!("Original {:?}", seq);
    for length in length_list {
        reverse_span(&mut seq, offset, length);
        offset += ((length + skipSize) % seq.len() as i32);
        skipSize += 1;
    }
    println!("Reverse {:?}", seq);
    println!("Part 1: {}*{} = {}", seq[0], seq[1], seq[0] * seq[1]);


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

}
