use std::cmp::max;
use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut split = input.trim().strip_prefix("target area: ").unwrap()
        .split(", ");
    // Assume xs are +ve and ys are -ve
    let xs = parse_bounds(split.next().unwrap());
    let ys = parse_bounds(split.next().unwrap());

    let res = solve1(xs, ys);
    println!("{}", res);

    let res = solve2(xs, ys);
    println!("{}", res);
}

fn solve1(_xs: (i32,i32), ys: (i32,i32)) -> i32 {
    let dy0 = get_max_dy(ys);
    dy0 * (dy0 + 1) / 2
}

fn solve2(xs: (i32,i32), ys: (i32,i32)) -> i32 {
    let mut res = 0;
    for dx in 1..xs.1+1 {
        for dy in ys.0..get_max_dy(ys)+1 {
            let mut x = 0;
            let mut y = 0;
            let mut dx = dx;
            let mut dy = dy;
            res += loop {
                x += dx;
                y += dy;
                dx = max(0, dx - 1);
                dy -= 1;

                if x > xs.1 || y < ys.0 {
                    break 0;
                }
                if x >= xs.0 && y  <= ys.1 {
                    break 1;
                }
            }
        }
    }

    res
}

fn parse_bounds(input: &str) -> (i32,i32) {
    let mut input = input[2..].split("..")
        .map(|s| s.parse::<i32>().unwrap());

    (input.next().unwrap(), input.next().unwrap())
}

/// Get maximum initial y velocity
fn get_max_dy(ys: (i32,i32)) -> i32 {
    // The fastest speed goes directly from the start position to the bottom
    // of the box.

    -ys.0 - 1
}
