use std::collections::*;
use std::io::stdin;

type Line = ((i32,i32),(i32,i32));

fn main() {
    let input = get_input();

    let res = solve1(&input);
    println!("{}", res);

    let res = solve2(&input);
    println!("{}", res);
}

fn solve1(lines: &[Line]) -> i32 {
    solve(lines, |(pos0,pos1)| pos0.0 == pos1.0 || pos0.1 == pos1.1)
}

fn solve2(lines: &[Line]) -> i32 {
    solve(lines, |_| true)
}

fn solve(lines: &[Line], filter: fn(&&Line) -> bool) -> i32 {
    let mut map: HashMap<(i32,i32),i32> = HashMap::new();
    let mut res = 0;
    for (pos0,pos1) in lines.iter().filter(filter) {
        let dir = (
            (pos1.0 - pos0.0).signum(),
            (pos1.1 - pos0.1).signum(),
        );

        let mut pos = *pos0;
        loop {
            let entry = map.entry((pos.0, pos.1)).or_insert(0);
            *entry += 1;
            if *entry == 2 {
                res += 1;
            }

            if pos == *pos1 {
                break;
            }
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }

    res
}

fn get_input() -> Vec<Line> {
    let stdin = stdin();
    let mut input = String::new();
    let mut res = Vec::new();
    loop {
        match stdin.read_line(&mut input).unwrap() {
            0 => break,
            _ => {
                res.push(parse_line(&input));
                input.clear();
            },
        }
    }

    res
}

fn parse_line(line: &str) -> Line {
    let line = line.trim();
    let split: Vec<_> = line.split(" -> ")
        .map(|s| parse_pair(s)).collect();
    (split[0], split[1])
}

fn parse_pair(input: &str) -> (i32,i32) {
    let split: Vec<i32> = input.split(',')
        .map(|s| s.parse().unwrap()).collect();
    (split[0], split[1])
}
