use std::io::{Read,stdin};

fn main() {
    let mut stdin = stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let input: Vec<Vec<_>> = input.split_whitespace()
        .map(|s| s.bytes()
             .map(|b| b - '0' as u8)
             .collect())
        .collect();

    let res = solve1(&input);
    println!("{}", res);
}

fn solve1(input: &[Vec<u8>]) -> i32 {
    let mut input = Vec::from(input);

    if input.len() == 0 {
        return 0;
    }

    let mut res = 0;
    for _ in 0..100 {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                inc(&mut input, i as i32, j as i32);
            }
        }
        res += zero(&mut input);
    }

    res
}

/// Increment the energy level at (i,j)
fn inc(input: &mut [Vec<u8>], i: i32, j: i32) {
    if i < 0 || j < 0 || i >= input.len() as i32 ||
        j >= input[0].len() as i32 ||
        input[i as usize][j as usize] == 10 {
        return;
    }

    input[i as usize][j as usize] += 1;
    if input[i as usize][j as usize] != 10 {
        return;
    }

    // Flash

    for i2 in i-1..i+2 {
        for j2 in j-1..j+2 {
            if i2 == i && j2 == j {
                continue;
            }
            inc(input, i2, j2);
        }
    }
}

/// Set 10s to 0s, and return how many there were
fn zero(input: &mut [Vec<u8>]) -> i32 {
    let mut res = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 10 {
                input[i][j] = 0;
                res += 1
            }
        }
    }

    res
}
