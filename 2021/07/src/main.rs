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
    let mut input: Vec<i32> = Vec::from(input);
    input.sort();
    let lo = input[0] as usize;
    let hi = *input.last().unwrap() as usize;

    // First get the fuel required for the crabs to the right
    // of each position, by iterating backwards

    let mut index = (input.len() - 1) as i32;
    let mut right = vec![0; hi - lo + 1];
    // Count crabs
    let mut count1 = 0;
    // Fuel per pos
    let mut count2 = 0;
    // Total fuel up to the current pos
    let mut count3 = 0;
    for (pos, val) in right.iter_mut().enumerate().rev() {
        let pos = pos + lo;
        count2 += count1;
        count3 += count2;
        while index >= 0 && pos == input[index as usize] as usize {
            index -= 1;
            count1 += 1;
        }

        *val = count3;
    }

    let mut res = 1000000000;
    let mut index = 0;
    // Count crabs
    let mut count1 = 0;
    // Fuel per pos
    let mut count2 = 0;
    // Total fuel up to the current pos
    let mut count3 = 0;
    for (pos, val) in right.iter().enumerate() {
        let pos = pos + lo;
        count2 += count1;
        count3 += count2;
        while index < input.len() && pos == input[index] as usize {
            index += 1;
            count1 += 1;
        }

        res = min(res, count3 + val);
    }

    res
}
