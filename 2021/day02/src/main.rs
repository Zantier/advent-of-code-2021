use std::io::{Read,stdin};

enum Command {
    Down(i32),
    Forward(i32),
}

fn main() {
    let mut input = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut input).unwrap();

    let input: Vec<_> = input.trim().lines().collect();

    let res = solve1(&input);
    println!("{}", res);

    let res = solve2(&input);
    println!("{}", res);
}

fn solve1(lines: &[&str]) -> i32 {
    let mut x = 0;
    let mut depth = 0;
    for line in lines {
        let pos = parse_command(line).unwrap();
        match pos {
            Command::Down(val) => depth += val,
            Command::Forward(val) => x += val,
        }
    }

    return x * depth;
}

fn solve2(lines: &[&str]) -> i32 {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let pos = parse_command(line).unwrap();
        match pos {
            Command::Down(val) => aim += val,
            Command::Forward(val) => {
                x += val;
                depth += val * aim;
            },
        }
    }

    return x * depth;
}

fn parse_command(command: &str) -> Option<Command> {
    let fields: Vec<_> = command.split_whitespace().collect();
    let val: i32 = fields[1].parse().unwrap();
    match fields[0] {
        "forward" => Some(Command::Forward(val)),
        "down" => Some(Command::Down(val)),
        "up" => Some(Command::Down(-val)),
        _ => None,
    }
}
