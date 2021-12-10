use std::collections::HashMap;
use std::io::{Read,stdin};

fn main() {
    let mut stdin = stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let input: Vec<_> = input.split_whitespace()
        .map(|s| s.as_bytes())
        .collect();

    let res = solve1(&input);
    println!("{}", res);

    let res = solve2(&input);
    println!("{}", res);
}

fn solve1(input: &[&[u8]]) -> i32 {
    let map: HashMap<_, (usize, i32)> = HashMap::from([
        (b'(', (0,1)),
        (b')', (0,-1)),
        (b'[', (1,1)),
        (b']', (1,-1)),
        (b'{', (2,1)),
        (b'}', (2,-1)),
        (b'<', (3,1)),
        (b'>', (3,-1)),
    ]);
    let points = [3, 57, 1197, 25137];
    let mut res = 0;
    for line in input {
        let mut stack: Vec<usize> = Vec::new();
        for ch in line.iter() {
            let &(index, val) = map.get(ch).unwrap();
            if val == 1 {
                stack.push(index);
            } else {
                if stack.pop() != Some(index) {
                    res += points[index];
                    break;
                }
            }
        }
    }

    res
}

fn solve2(input: &[&[u8]]) -> i64 {
    let map: HashMap<_, (usize, i32)> = HashMap::from([
        (b'(', (0,1)),
        (b')', (0,-1)),
        (b'[', (1,1)),
        (b']', (1,-1)),
        (b'{', (2,1)),
        (b'}', (2,-1)),
        (b'<', (3,1)),
        (b'>', (3,-1)),
    ]);
    let mut scores: Vec<i64> = Vec::new();
    'outer: for line in input {
        let mut stack: Vec<usize> = Vec::new();
        for ch in line.iter() {
            let &(index, val) = map.get(ch).unwrap();
            if val == 1 {
                stack.push(index);
            } else {
                if stack.pop() != Some(index) {
                    // Corrupted
                    continue 'outer;
                }
            }
        }

        // Not a corrupted line

        let mut score = 0;
        while let Some(top) = stack.pop() {
            score = 5*score + (top as i64 + 1);
        }
        scores.push(score);
    }

    // Return median score
    scores.sort();
    scores[(scores.len()-1)/2]
}
