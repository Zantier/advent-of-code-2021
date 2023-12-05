use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{Read,stdin};

fn main() {
    let mut stdin = stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let grid: Vec<_> = input.split_whitespace()
        .map(|s| s.as_bytes().iter()
             .map(|b| b - b'0')
             .collect())
        .collect();

    let res = solve1(&grid);
    println!("{}", res);

    let res = solve2(&grid);
    println!("{}", res);
}

fn solve1(grid: &[Vec<u8>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![false;width];height];

    heap.push(Reverse((0,(0 as i32,0 as i32))));
    loop {
        let Reverse((val,(i1,j1))) = heap.pop().unwrap();
        if visited[i1 as usize][j1 as usize] {
            continue;
        }

        visited[i1 as usize][j1 as usize] = true;

        if i1 == height as i32 - 1 && j1 == width as i32 - 1 {
            break val;
        }

        let adjs = [(i1+1,j1),(i1-1,j1),(i1,j1+1),(i1,j1-1)];
        for (i,j) in adjs {
            if i >= 0 && j >= 0 && i < height as i32 && j < width as i32 {
                heap.push(Reverse((val + grid[i as usize][j as usize] as i32, (i,j))));
            }
        }
    }
}

fn solve2(grid: &[Vec<u8>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![vec![vec![false;5];5];width];height];

    heap.push(Reverse((0,(0 as i32,0 as i32, 0 as i32, 0 as i32))));
    loop {
        let Reverse((val,(i1,j1,k1,l1))) = heap.pop().unwrap();
        if visited[i1 as usize][j1 as usize][k1 as usize][l1 as usize]  {
            continue;
        }

        visited[i1 as usize][j1 as usize][k1 as usize][l1 as usize] = true;

        if i1 == height as i32 - 1 && j1 == width as i32 - 1 && k1 == 4 && l1 == 4 {
            break val;
        }

        let adjs = get_adjs(i1,j1,k1,l1, width as i32, height as i32);
        for (i,j,k,l) in adjs {
            if i >= 0 && j >= 0 && k >= 0 && l >= 0 && i < height as i32 &&
                j < width as i32 && k < 5 && l < 5 {
                let grid_val = wrap(grid[i as usize][j as usize] as i32 + k + l);
                heap.push(Reverse((val + grid_val, (i,j,k,l))));
            }
        }
    }
}

fn wrap(val: i32) -> i32 {
    (val - 1) % 9 + 1
}

fn get_adjs(i: i32, j: i32, k: i32, l: i32, height: i32, width: i32) -> [(i32,i32,i32,i32);4] {
    let mut res = [(i+1,j,k,l),(i-1,j,k,l),(i,j+1,k,l),(i,j-1,k,l)];
    for x in res.iter_mut() {
        *x = normalize(*x, height, width);
    }

    res
}

fn normalize(tup: (i32,i32,i32,i32), height: i32, width: i32) -> (i32,i32,i32,i32) {
    let mut res = tup;
    if res.0 < 0 {
        res.0 += height;
        res.2 -= 1;
    }
    if res.0 >= height {
        res.0 -= height;
        res.2 += 1;
    }
    if res.1 < 0 {
        res.1 += width;
        res.3 -= 1;
    }
    if res.1 >= width {
        res.1 -= width;
        res.3 += 1;
    }

    res
}
