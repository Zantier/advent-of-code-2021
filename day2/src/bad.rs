use std::io::{Read,stdin};

struct Pos {
    x: i32,
    depth: i32,
}

fn main() {
    let mut input = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut input).expect("oops");

    let input: Vec<_> = input.trim().lines().collect();
    let res = solve(&input);
    println!("{}", res);
}

fn solve(lines: &[&str]) -> i32 {
    let mut res = Pos { x: 0, depth: 0 };
    for line in lines {
        let pos = parse_command(line).unwrap();
        res.x += pos.x;
        res.depth += pos.depth;
    }

    return res.x * res.depth;
}

fn parse_command(command: &str) -> Option<Pos> {
    let fields: Vec<_> = command.split_whitespace().collect();
    let val: i32 = fields[1].parse().unwrap();
    match fields[0] {
        "forward" => Some(Pos { x: val, depth: 0 }),
        "down" => Some(Pos { x: 0, depth: val }),
        "up" => Some(Pos { x: 0, depth: -val }),
        _ => None,
    }
}
