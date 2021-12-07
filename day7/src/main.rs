use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let stdin = stdin();
    stdin.read_line(&mut input).unwrap();

    let input: Vec<i32> = input.trim().split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let res = solve1(&input);
    println!("{}", res);

    let res = solve2(&input);
    println!("{}", res);
}

fn solve1(input: &[i32]) -> i32 {
    // Get the median
    let mut input: Vec<i32> = Vec::from(input);
    input.sort();
    let mid = (input.len()-1) / 2;
    let mid = input[mid];

    let res: i32 = input.iter()
        .map(|num| (num-mid).abs())
        .sum();
    res
}

fn solve2(input: &[i32]) -> i32 {
    // Binary search on horizontal position
    let lo = *input.iter().min().unwrap();
    let hi = *input.iter().max().unwrap();

    let mut res = 1000000000;
    for pos in lo..hi+1 {
        let val = get_fuel(input, pos);
        res = min(res, val);
    }
    res
}

/// Get the fuel needed for all crabs to get to pos
fn get_fuel(input: &[i32], pos: i32) -> i32 {
    let mut res = 0;
    for val in input.iter() {
        let val = (pos - val).abs();
        res += (val * (val+1)) / 2;
    }
    res
}
