use std::collections::VecDeque;
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

    let res = solve2(&input);
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

fn solve2(grid: &[Vec<u8>]) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut res: Vec<i32> = Vec::new();
    let mut visited = vec![vec![false; width as usize]; height as usize];
    for i in 0..height {
        for j in 0..width {
            let min = [(i+1,j),(i-1,j),(i,j+1),(i,j-1)].iter()
                .map(|x| val(grid, x.0, x.1))
                .min().unwrap();
            let val = grid[i as usize][j as usize] as i32;
            if val < min {
                // low point
                let count = bfs(grid, &mut visited, i, j);
                res.push(count);
            }
        }
    }

    res.sort_by_key(|x| -x);
    res[0] * res[1] * res[2]
}

fn bfs(grid: &[Vec<u8>], visited: &mut [Vec<bool>], i: i32, j: i32) -> i32 {
    let mut queue: VecDeque<(i32,i32)> = VecDeque::new();
    queue.push_back((i,j));
    let mut res = 0;
    while !queue.is_empty() {
        let (i,j) = queue.pop_back().unwrap();
        if val(grid, i, j) >= 9 {
            continue;
        }
        if visited[i as usize][j as usize] {
            continue;
        }

        res += 1;
        visited[i as usize][j as usize] = true;

        for (i2,j2) in [(i+1,j),(i-1,j),(i,j+1),(i,j-1)] {
            queue.push_back((i2,j2));
        }
    }

    res
}
