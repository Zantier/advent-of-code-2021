use std::io::stdin;

fn main() {
    let poses = get_starting_positions();

    let res = solve1(poses);
    println!("{}", res);
}

fn solve1(poses: [i32;2]) -> i32 {
    let mut poses = poses;
    let mut scores = [0;2];
    let mut index = 0;
    let mut dice = 1;
    let mut roll_count = 0;
    loop {
        roll_count += 3;
        for _ in 0..3 {
            poses[index] += dice;
            dice += 1;
        }

        // 1-10 inclusive
        poses[index] = (poses[index] - 1) % 10 + 1;
        scores[index] += poses[index];
        if scores[index] >= 1000 {
            break;
        }

        index = 1 - index;
    }

    scores[1-index] * roll_count
}

fn get_starting_positions() -> [i32;2] {
    let stdin = stdin();

    let mut res = [0;2];
    for i in 0..res.len() {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        res[i] = get_starting_position(input.trim());
    }

    res
}

fn get_starting_position(input: &str) -> i32 {
    let ch = input.as_bytes().last().unwrap();
    let res = ch - b'0';
    res as i32
}
