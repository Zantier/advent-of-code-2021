use std::io;


const NAMES: [&str;9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];


fn main() {
    let lines: Vec<_> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let res = solve1(&lines);
    println!("{}", res);

    let res = solve2(&lines);
    println!("{}", res);
}


fn solve1(lines: &[String]) -> u32 {
    let mut res = 0;

    for line in lines {
        let mut digit1: Option<u32> = None;
        let mut digit2 = 0;
        for char in line.chars() {
            if char >= '0' && char <= '9' {
                let num = char.to_digit(10).unwrap();
                digit1.get_or_insert(num);
                digit2 = num;
            }
        }

        let digit1 = digit1.unwrap_or(0);
        res += digit1 * 10 + digit2;
    }

    res
}


fn solve2(lines: &[String]) -> u32 {
    let mut res = 0;

    for line in lines {
        let mut digit1: Option<u32> = None;
        let mut digit2 = 0;
        for (pos, char) in line.char_indices() {
            if char >= '0' && char <= '9' {
                let num = char.to_digit(10).unwrap();
                digit1.get_or_insert(num);
                digit2 = num;
            }

            for (i, &name) in NAMES.iter().enumerate() {
                let i = i as u32 + 1;
                if line[pos..].starts_with(name) {
                    digit1.get_or_insert(i);
                    digit2 = i;
                }
            }
        }

        let digit1 = digit1.unwrap_or(0);
        res += digit1 * 10 + digit2;
    }

    res
}
