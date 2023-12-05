use std::cmp::max;
use std::fmt;
use std::io::stdin;

#[derive(Clone,Debug)]
enum Number {
    Num(i32),
    Pair(Box<(Number,Number)>),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Num(val) => {
                write!(f, "{}", val)
            },
            Number::Pair(pair) => {
                write!(f, "[{},{}]", pair.0, pair.1)
            },
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut nums = Vec::new();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let num = parse_number(input.trim().as_bytes(), &mut 0);
        nums.push(num);
    }

    let res = solve1(&nums);
    println!("{}", res);

    let res = solve2(&nums);
    println!("{}", res);
}

fn solve1(nums: &[Number]) -> i32 {
    let mut res = nums[0].clone();
    for num in nums.iter().skip(1) {
        res = Number::Pair(Box::new((res,num.clone())));
        reduce(&mut res);
    }

    magnitude(&res)
}

fn solve2(nums: &[Number]) -> i32 {
    let mut res = 0;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let sum = solve1(&[nums[i].clone(),nums[j].clone()]);
            res = max(res, sum);

            let sum = solve1(&[nums[j].clone(),nums[i].clone()]);
            res = max(res, sum);
        }
    }

    res
}

fn parse_number(input: &[u8], index: &mut usize) -> Number {
    if input[*index] == b'[' {
        *index += 1;
        let num1 = parse_number(input, index);
        *index += 1;
        let num2 = parse_number(input, index);
        *index += 1;
        Number::Pair(Box::new((num1,num2)))
    } else {
        // integer

        let num = input[*index] - b'0';
        *index += 1;
        Number::Num(num as i32)
    }
}

fn reduce(num: &mut Number) {
    loop {
        let res = explode(num, 0);
        if res.2 {
            continue;
        }

        // Didn't explode, so try split
        if !split(num) {
            break;
        }
    }
}

fn explode(num: &mut Number, level: i32) -> (i32,i32,bool) {
    if let Number::Pair(pair) = num {
        if level == 4 {
            // Assume both branches are just numbers
            let left = match pair.0 {
                Number::Num(num) => num,
                _ => panic!("oops"),
            };
            let right = match pair.1 {
                Number::Num(num) => num,
                _ => panic!("oops"),
            };

            return (left,right,true);
        }

        let left = explode(&mut pair.0, level + 1);
        if left.2 {
            if level == 3 {
                pair.0 = Number::Num(0);
            }

            if left.1 > 0 {
                explode_add_first(&mut pair.1, left.1);
            }

            return (left.0,0,true);
        }

        let right = explode(&mut pair.1, level + 1);
        if right.2 {
            if level == 3 {
                pair.1 = Number::Num(0);
            }

            if right.0 > 0 {
                explode_add_last(&mut pair.0, right.0);
            }

            return (0,right.1,true);
        }
    }

    (0,0,false)
}

/// Perform 1 split
fn split(num: &mut Number) -> bool {
    match num {
        Number::Num(val) => {
            if *val >= 10 {
                *num = Number::Pair(Box::new((
                    Number::Num(*val / 2),
                    Number::Num(*val / 2 + *val % 2),
                )));
                true
            } else {
                false
            }
        },
        Number::Pair(pair) => {
            split(&mut pair.0) || split(&mut pair.1)
        },
    }
}

/// Add to the first value in the number
fn explode_add_first(num: &mut Number, val: i32) {
    match num {
        Number::Num(num) => *num += val,
        Number::Pair(pair) => explode_add_first(&mut pair.0, val),
    }
}

/// Add to the last value in the number
fn explode_add_last(num: &mut Number, val: i32) {
    match num {
        Number::Num(num) => *num += val,
        Number::Pair(pair) => explode_add_last(&mut pair.1, val),
    }
}

fn magnitude(num: &Number) -> i32 {
    match num {
        Number::Num(num) => *num,
        Number::Pair(pair) => 3*magnitude(&pair.0) + 2*magnitude(&pair.1),
    }
}
