use std::io::{stdin,Stdin};

const SIZE: i32 = 5;

/// A bingo board. The first index is the row.
#[derive(Debug)]
struct Board {
    vals: Vec<Vec<i32>>,
    marks: Vec<Vec<bool>>,
    rows_left: Vec<i32>,
    cols_left: Vec<i32>,
}

impl Board {
    fn new(vals: Vec<Vec<i32>>) -> Board {
        let vec_size = SIZE as usize;
        Board {
            vals: vals,
            marks: vec![vec![false; vec_size]; vec_size],
            rows_left: vec![SIZE; vec_size],
            cols_left: vec![SIZE; vec_size],
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut stdin = stdin();
    stdin.read_line(&mut input).unwrap();
    input.pop();

    let nums: Vec<i32> = input.split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    // Read blank line
    stdin.read_line(&mut input).unwrap();

    let mut boards = get_stdin_boards(&mut stdin);

    let res = solve1(&nums, &mut boards);
    println!("{}", res);
}

fn solve1(nums: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for num in nums.iter() {
        for board in boards.iter_mut() {
            for (i, row) in board.vals.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if val == num {
                        board.marks[i][j] = true;
                        board.rows_left[i] -= 1;
                        board.cols_left[j] -= 1;

                        if board.rows_left[i] == 0 || board.cols_left[j] == 0 {
                            return num * board_val(board);
                        }
                    }
                }
            }
        }
    }

    0
}

fn board_val(board: &Board) -> i32 {
    let mut res = 0;
    for (i, row) in board.vals.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if !board.marks[i][j] {
                res += val;
            }
        }
    }

    return res;
}

fn get_stdin_boards(stdin: &mut Stdin) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut input = String::new();
    loop {
        // Check if there's another board to read
        match stdin.read_line(&mut input).unwrap() {
            0 => break,
            _ => {
                let mut rows: Vec<Vec<i32>> = Vec::new();
                let row = parse_board_row(&input);
                rows.push(row);

                for _ in 0..SIZE-1 {
                    input.clear();
                    stdin.read_line(&mut input).unwrap();
                    let row = parse_board_row(&input);
                    rows.push(row);
                }

                // Read blank line
                stdin.read_line(&mut input).unwrap();
                input.clear();

                boards.push(Board::new(rows));
            },
        }
    }

    boards
}

fn parse_board_row(input: &str) -> Vec<i32> {
    input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
