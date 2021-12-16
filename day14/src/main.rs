use std::collections::HashMap;
use std::io::stdin;

struct Rule {
    first: u8,
    second: u8,
    new: u8,
}

fn main() {
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let chars = input.trim().as_bytes();

    // Blank line
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut rules: HashMap<(u8,u8), u8> = HashMap::new();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let rule = parse_rule(input.trim());
        rules.insert((rule.first, rule.second), rule.new);
    }

    let res = solve1(chars, &rules);
    println!("{}", res);

    let res = solve2(chars, &rules);
    println!("{}", res);
}

fn solve1(chars: &[u8], rules: &HashMap<(u8,u8), u8>) -> i64 {
    solve(chars, rules, 10)
}

fn solve2(chars: &[u8], rules: &HashMap<(u8,u8), u8>) -> i64 {
    solve(chars, rules, 40)
}

fn solve(chars: &[u8], rules: &HashMap<(u8,u8), u8>, steps: i32) -> i64 {
    let single_chars = [*chars.first().unwrap(), *chars.last().unwrap()];
    let mut pairs: HashMap<_,i64> = HashMap::new();
    for (&ch1,&ch2) in chars.iter().zip(chars.iter().skip(1)) {
        let key = (ch1,ch2);
        *pairs.entry(key).or_insert(0) += 1
    }

    for _ in 0..steps {
        let mut new_pairs = pairs.clone();
        for (pair,count) in pairs {
            if let Some(&new) = rules.get(&pair) {
                *new_pairs.entry(pair).or_insert(0) -= count;
                *new_pairs.entry((pair.0,new)).or_insert(0) += count;
                *new_pairs.entry((new,pair.1)).or_insert(0) += count;
            }
        }

        pairs = new_pairs;
    }

    let mut counts = HashMap::new();
    // By counting the chars in each pair, we double count each char,
    // apart from first and last
    for ((ch1,ch2),count) in pairs {
        *counts.entry(ch1).or_insert(0) += count;
        *counts.entry(ch2).or_insert(0) += count;
    }

    *counts.entry(single_chars[0]).or_default() += 1;
    *counts.entry(single_chars[1]).or_default() += 1;

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    (max - min) / 2
}



fn parse_rule(input: &str) -> Rule {
    let fields: Vec<_> = input.split(" -> ")
        .map(|s| s.as_bytes())
        .collect();

    Rule {
        first: fields[0][0],
        second: fields[0][1],
        new: fields[1][0],
    }
}
