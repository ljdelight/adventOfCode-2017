
fn compute(value: i64, factor: i64) -> i64 {
    (value * factor) % 2147483647
}

fn generate(value: i64, factor: i64, div: i64) -> i64 {
    let mut res = value;
    loop {
        res = compute(res, factor);
        if res % div == 0 {
            return res;
        }
    }
}

fn main() {
    let mut a_input = 591;
    let a_factor = 16807;
    let mut b_input = 393;
    let b_factor = 48271;

    // P1 assertions
    assert!(compute(8921, b_factor) == 430625591);
    assert!(compute(1181022009, a_factor) == 245556042);
    assert!(compute(1181022009, a_factor) & 0xffff == compute(1233683848, b_factor) & 0xffff);

    // P2 assertions
    assert!(generate(65, a_factor, 4) == 1352636452);
    assert!(generate(1352636452, a_factor, 4) == 1992081072);
    assert!(generate(8921, b_factor, 8) == 1233683848);


    let mut count = 0;
    let mut a_last = a_input;
    let mut b_last = b_input;
    for _ in 0..40_000_000 {
        a_last = compute(a_last, a_factor);
        b_last = compute(b_last, b_factor);
        if a_last & 0xffff == b_last & 0xffff {
            count += 1;
        }
    }
    println!("Part 1: Found {} bit matches", count);

    let mut count = 0;
    let mut a_last = a_input;
    let mut b_last = b_input;
    for _ in 0..5_000_000 {
        a_last = generate(a_last, a_factor, 4);
        b_last = generate(b_last, b_factor, 8);
        if a_last & 0xffff == b_last & 0xffff {
            count += 1;
        }
    }
    println!("Part 2: Found {} bit matches", count);
}
