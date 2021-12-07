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
