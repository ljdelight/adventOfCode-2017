
//
// Part 1:
// - Let g be the 'radius' from the center.
// - Let N_g be the number of elements included in radius g.
//    Example: When g=2, the 5x5 matrix has 16 elements in its outermost ring.
//    N_g = 8g
// - Let S_g be the total number of elements included in radius g through radius 1.
//    Example: When g=2, the 5x5 matrix has 24 elements in all rings
//      (5x5 matrix is 25 elements, then subtract the center).
//    S_g = N_g + S_(g-1)
//        = N_g + N_(g-1) + N_(g-2) + ... + N_1
//        = 8g  + 8(g-1)  + 8(g-2)  + ... + 8
//        = 8(g +  (g-1)  +  (g-2)  + ... + 1)
//        = 4g(g+1)
// - Let H_g be the 'height' of radius g, which is the length of the square with radius g.
//    Example: When g=2, there is a 5x5 matrix of elements and H_2=5.
//    H_g = 2g+1
//
// INPUT = 347991
// Using INPUT as S_g and solving for an upperbound g, we determine that g = 295.
//
// S_g     = S_295 = 349280
// S_(g-1) = S_294 = 346920
// H_g = H_295 = 591
// H_g = H_294 = 589
//

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rotate(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn update_sum(matrix: &mut [[u32; 1024]; 1024], i: usize, j: usize) {
    // U D L R
    matrix[i][j] = matrix[i - 1][j] + matrix[i + 1][j] + matrix[i][j - 1] + matrix[i][j + 1];
    // Corners UR UL LR LL
    matrix[i][j] += matrix[i - 1][j + 1] + matrix[i - 1][j - 1] + matrix[i + 1][j + 1] +
        matrix[i + 1][j - 1];
}


fn main() {
    let mut matrix = [[0u32; 1024]; 1024];
    matrix[512][512] = 1;
    matrix[512][513] = 1;

    let mut limit = 3;
    let mut count = 0;
    let mut i = 511;
    let mut j = 513;
    let mut direction = Direction::Left;

    loop {
        update_sum(&mut matrix, i, j);
        count += 1;
        if matrix[i][j] > 347991 {
            println!(
                "FOUND ({},{})={} to be greater than puzzle input",
                i,
                j,
                matrix[i][j]
            );
            break;
        }
        // println!("({},{}) = {}", i, j, matrix[i][j]);


        if count == limit {
            count = 1;
            direction = rotate(direction);
            // println!("Changed direction to {:?}", direction);

            // If an entire 'square' was filled out, prepare for the next square.
            match direction {
                Direction::Up => {
                    limit += 2;
                    j += 1;
                    continue;
                }
                _ => (),
            }
        }
        match direction {
            Direction::Up => {
                i -= 1;
            }
            Direction::Down => {
                i += 1;
            }
            Direction::Left => {
                j -= 1;
            }
            Direction::Right => {
                j += 1;
            }
        }
    }
}
