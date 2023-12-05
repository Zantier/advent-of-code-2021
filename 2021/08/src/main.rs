use std::collections::HashMap;
use std::io::stdin;

struct Display {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let stdin = stdin();
    let mut data: Vec<Display> = Vec::new();
    loop  {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut split_bar = input.split('|');
        let patterns: Vec<_> = split_bar.next().unwrap().trim()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();
        let output: Vec<_> = split_bar.next().unwrap().trim()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();

        data.push(Display {
            patterns,
            output,
        });
    }

    let res = solve1(&data);
    println!("{}", res);

    let res = solve2(&data);
    println!("{}", res);
}

fn solve1(input: &[Display]) -> i32 {
    let mut res = 0;
    for display in input.iter() {
        for s in display.output.iter() {
            if [2,3,4,7].contains(&(s.len() as i32)) {
                res += 1;
            }
        }
    }
    res
}

fn solve2(input: &[Display]) -> i32 {
    let mut res = 0;

    for display in input.iter() {
        // Map 7-segment digit to bit pattern
        let mut patterns = vec![0; 10];
        // Map from bit pattern to 7-segment digit
        let mut map: HashMap<i32,i32> = HashMap::new();
        let unordered_patterns: Vec<_> = display.patterns.iter()
            .map(|s| to_bits(s))
            .collect();
        let output: Vec<_> = display.output.iter()
            .map(|s| to_bits(s))
            .collect();

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 2).next().unwrap();
        patterns[1] = *bits;
        map.insert(*bits, 1);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 3).next().unwrap();
        patterns[7] = *bits;
        map.insert(*bits, 7);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 4).next().unwrap();
        patterns[4] = *bits;
        map.insert(*bits, 4);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 7).next().unwrap();
        patterns[8] = *bits;
        map.insert(*bits, 8);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 5 && (*x & patterns[1]).count_ones() == 2)
            .next().unwrap();
        patterns[3] = *bits;
        map.insert(*bits, 3);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 5 &&
                (*x & patterns[1]).count_ones() == 1 &&
                (*x & patterns[4]).count_ones() == 2
            ).next().unwrap();
        patterns[2] = *bits;
        map.insert(*bits, 2);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 5 &&
                (*x & patterns[1]).count_ones() == 1 &&
                (*x & patterns[4]).count_ones() == 3
            ).next().unwrap();
        patterns[5] = *bits;
        map.insert(*bits, 5);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 6 && (*x & patterns[5]).count_ones() == 4)
            .next().unwrap();
        patterns[0] = *bits;
        map.insert(*bits, 0);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 6 && (*x | patterns[1]).count_ones() == 7)
            .next().unwrap();
        patterns[6] = *bits;
        map.insert(*bits, 6);

        let bits = unordered_patterns.iter()
            .filter(|x| x.count_ones() == 6 && (*x & patterns[4]).count_ones() == 4)
            .next().unwrap();
        patterns[9] = *bits;
        map.insert(*bits, 9);

        let mut tmp = 0;
        for i in output.iter() {
            let val = map.get(i).unwrap();
            tmp = 10*tmp + val;
        }

        res += tmp;
    }

    res
}

fn to_bits(s: &str) -> i32 {
    let mut res = 0;
    for ch in s.chars() {
        let bit = ch as u8 - 'a' as u8;
        res |= 1 << bit;
    }
    res
}
