use std::collections::*;
use std::io::stdin;

fn main() {
    let input = get_input();
    let mut map: HashMap<(i32,i32),i32> = HashMap::new();
    let mut res = 0;
    for (pos0,pos1) in input.iter() {
        if pos0.0 == pos1.0 {
            let lo = pos0.1.min(pos1.1);
            let hi = pos0.1.max(pos1.1);
            for i in lo..hi+1 {
                let entry = map.entry((pos0.0, i)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    res += 1;
                }
            }
        } else if pos0.1 == pos1.1 {
            let lo = pos0.0.min(pos1.0);
            let hi = pos0.0.max(pos1.0);
            for i in lo..hi+1 {
                let entry = map.entry((i, pos0.1)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}

fn get_input() -> Vec<((i32,i32),(i32,i32))> {
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

fn parse_line(line: &str) -> ((i32,i32),(i32,i32)) {
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
