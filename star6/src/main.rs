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
    let lines_bits: Vec<_> = lines.iter().map(|s| to_bits(s))
        .collect();

    let o2 = get_rating(&lines_bits, criteria_o2);
    let co2 = get_rating(&lines_bits, criteria_co2);

    let o2 = i32::from_str_radix(lines[o2], 2).unwrap();
    let co2 = i32::from_str_radix(lines[co2], 2).unwrap();

    o2 * co2
}

fn get_rating(lines: &Vec<Vec<u8>>, criteria: fn(i32,i32) -> u8) -> usize {
    let len = lines[0].len();
    let mut done = vec![false; lines.len()];
    let mut values_left = lines.len();
    for i in 0..len {
        let mut count0 = 0;
        let mut count1 = 0;
        for (j,line) in lines.iter().enumerate() {
            if !done[j] {
                if line[i] == 0 {
                    count0 += 1;
                } else {
                    count1 += 1;
                }
            }
        }

        // We want to keep values with this bit
        let good_bit = criteria(count0, count1);
        for (j,line) in lines.iter().enumerate() {
            if !done[j] {
                if line[i] != good_bit {
                    done[j] = true;
                    values_left -= 1;
                }
            }
        }

        if values_left == 1 {
            break;
        }
    }

    for (i,val) in done.iter().enumerate() {
        if !val {
            return i;
        }
    }

    panic!("Failed to find rating");
}

fn criteria_o2(zeroes: i32, ones: i32) -> u8 {
    if ones >= zeroes {1} else {0}
}
fn criteria_co2(zeroes: i32, ones: i32) -> u8 {
    if ones >= zeroes {0} else {1}
}

fn to_bits(s: &str) -> Vec<u8> {
    s.chars().map(|c| if c == '0' {0} else {1})
        .collect()
}
