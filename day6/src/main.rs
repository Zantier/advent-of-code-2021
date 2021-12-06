use std::io::stdin;
fn main() {
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let values: Vec<i32> = input.trim().split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let res = solve1(&values);
    println!("{}", res);
}

fn solve1(values: &[i32]) -> i32 {
    let mut values = Vec::from(values);
    for _ in 0..80 {
        let mut new = 0;
        for val in values.iter_mut() {
            *val -= 1;
            if *val < 0 {
                *val = 6;
                new += 1;
            }
        }

        for _ in 0..new {
            values.push(8);
        }
    }

    values.len() as i32
}
