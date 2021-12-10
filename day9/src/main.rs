use std::io::{Read,stdin};
fn main() {
    let mut stdin = stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let input: Vec<Vec<_>> = input.split_whitespace()
        .map(|s| s.chars()
            .map(|ch| ch as u8 - '0' as u8)
            .collect())
        .collect();

    let res = solve1(&input);
    println!("{}", res);
}

fn val(grid: &[Vec<u8>], i: i32, j: i32) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    if i < 0 || j < 0 || i >= height || j >= width {
        return 1000000000;
    }
    return grid[i as usize][j as usize] as i32;
}

fn solve1(grid: &[Vec<u8>]) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut res = 0;
    for i in 0..height {
        for j in 0..width {
            let min = [(i+1,j),(i-1,j),(i,j+1),(i,j-1)].iter()
                .map(|x| val(grid, x.0, x.1))
                .min().unwrap();
            let val = grid[i as usize][j as usize] as i32;
            if val < min {
                res += val + 1;
            }
        }
    }

    res
}
