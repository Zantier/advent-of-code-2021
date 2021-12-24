use std::cmp::{max,min};
use std::collections::HashSet;
use std::io::stdin;

#[derive(Debug)]
struct Step {
    on: bool,
    xs: (i32,i32),
    ys: (i32,i32),
    zs: (i32,i32),
}

fn main() {
    let input = get_input();

    let res = solve1(&input);
    println!("{}", res);

    let res = solve2(&input);
    println!("{}", res);
}

fn solve1(steps: &[Step]) -> i32 {
    let mut set = HashSet::new();
    let limit = 50;
    for step in steps {
        for x in max(-limit,step.xs.0)..min(limit,step.xs.1)+1 {
            for y in max(-limit,step.ys.0)..min(limit,step.ys.1)+1 {
                for z in max(-limit,step.zs.0)..min(limit,step.zs.1)+1 {
                    if step.on {
                        set.insert([x,y,z]);
                    } else {
                        set.remove(&[x,y,z]);
                    }
                }
            }
        }
    }

    set.len() as i32
}

fn solve2(steps: &[Step]) -> i64 {
    0
}

fn get_input() -> Vec<Step> {
    let mut res = Vec::new();

    let stdin = stdin();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut split = input.split_whitespace();
        let on = split.next().unwrap() == "on";

        let mut split = split.next().unwrap().split(',');
        res.push(Step {
            on,
            xs: parse_range(split.next().unwrap()),
            ys: parse_range(split.next().unwrap()),
            zs: parse_range(split.next().unwrap()),
        });
    }

    res
}

fn parse_range(input: &str) -> (i32,i32) {
    let mut split = input[2..].split("..");
    (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    )
}
