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
