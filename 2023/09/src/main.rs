use std::io;


fn main() {
    let lines: Vec<_> = io::stdin().lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect();

    let res = solve1(&lines);
    println!("{}", res);

    let res = solve2(&lines);
    println!("{}", res);
}


fn solve1(lines: &[Vec<i32>]) -> i32 {
    lines.iter()
        .map(|line| solve1_line(line))
        .sum()
}


fn solve2(lines: &[Vec<i32>]) -> i32 {
    lines.iter()
        .map(|line| solve2_line(line))
        .sum()
}


fn solve1_line(line: &[i32]) -> i32 {
    let mut line = line.to_vec();
    let mut lasts = Vec::new();
    while !line.iter().all(|&x| x == 0) {
        for i in 0..line.len()-1 {
            line[i] = line[i+1] - line[i];
        }

        lasts.push(line.pop().unwrap());
    }

    lasts.iter().sum()
}


fn solve2_line(line: &[i32]) -> i32 {
    let mut line = line.to_vec();
    let mut firsts = Vec::new();
    while !line.iter().all(|&x| x == 0) {
        firsts.push(line[0]);
        for i in 0..line.len()-1 {
            line[i] = line[i+1] - line[i];
        }

        line.pop().unwrap();
    }

    let mut res = 0;
    loop {
        match firsts.pop() {
            Some(x) => res = x - res,
            None => return res,
        }
    }
}


fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect()
}
