use std::io::{Read,stdin};

fn main() {
    let mut input = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut input).unwrap();

    let nums = get_nums(&input);

    let res = solve1(&nums);
    println!("{}", res);

    let res = solve2(&nums);
    println!("{}", res);
}

fn solve1(nums: &[i32]) -> i32 {
    let res = nums.iter().zip(nums.iter().skip(1))
        .filter(|(a,b)| a < b)
        .count();

    res as i32
}

fn solve2(nums: &[i32]) -> i32 {
    let sums: Vec<i32> = nums.windows(3)
        .map(|x| x.iter().sum())
        .collect();

    let res = sums.iter().zip(sums.iter().skip(1))
        .filter(|(a,b)| a < b)
        .count();

    res as i32
}

/// Convert newline-delimited integers to a Vec
fn get_nums(text: &str) -> Vec<i32> {
    text.trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test_get_nums() {
    assert_eq!(get_nums("1\n2"), vec![1,2]);
}
