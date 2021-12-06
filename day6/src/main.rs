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

    let res = solve2(&values);
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

fn solve2(values: &[i32]) -> i64 {
    let days = 256;
    // After each day, how many new fish spawn
    let mut fish: Vec<i64> = Vec::new();
    fish.resize(20,0);
    for val in values.iter() {
        fish[(val+1) as usize] += 1;
    }

    for index in 0..days {
        if fish.len() < index + 10 {
            fish.resize(index + 10, 0);
        }
        fish[index+7] += fish[index];
        fish[index+9] += fish[index];
    }
    fish.iter().take(days+1).sum::<i64>() + values.len() as i64
}
