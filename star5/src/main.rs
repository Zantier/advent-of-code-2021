use std::io::{Read,stdin};

fn main() {
    let mut input = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut input).expect("oops");

    let input: Vec<_> = input.trim().lines().collect();
    let res = solve(&input);
    println!("{}", res);
}

fn solve(lines: &[&str]) -> i32 {
    let lines: Vec<_> = lines.iter().map(|s| to_bits(s))
        .collect();

    let len = lines[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..len {
        let mut count0 = 0;
        let mut count1 = 0;
        for line in &lines {
            if line[i] == 0 {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }

        if count0 > count1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn to_bits(s: &str) -> Vec<u8> {
    s.chars().map(|c| if c == '0' {0} else {1})
        .collect()
}
