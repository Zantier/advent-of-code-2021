use std::io;


#[derive(Debug)]
struct Number {
    num: u32,
    row: usize,
    start: usize,
    end: usize,
    include: bool,
}


impl Number {
    fn is_near(&self, row: usize, col: usize) -> bool {
        let row = row as i32;
        let col = col as i32;

        row >= self.row as i32 - 1 && row <= self.row as i32 + 1 &&
            col >= self.start as i32 - 1 && col <= self.end as i32
    }
}


fn main() {
    let lines: Vec<Vec<_>> = io::stdin().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let res = solve1(&lines);
    println!("{}", res);

    let res = solve2(&lines);
    println!("{}", res);
}


/// Create a list of numbers, then for each symbol, check each number
/// It should be more efficient check the initial input for nearby numbers,
/// but I can't be bothered
fn solve1(lines: &[Vec<char>]) -> u32 {
    let mut numbers = get_numbers(lines);

    for (line_i, line) in lines.iter().enumerate() {
        for (char_i, &char) in line.iter().enumerate() {
            if !('0'..='9').contains(&char) && char != '.' {
                for number in numbers.iter_mut() {
                    if number.is_near(line_i, char_i) {
                        number.include = true;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for number in &numbers {
        if number.include {
            res += number.num;
        }
    }

    res
}


fn solve2(lines: &[Vec<char>]) -> u32 {
    let mut numbers = get_numbers(lines);

    let mut res = 0;
    for (line_i, line) in lines.iter().enumerate() {
        for (char_i, &char) in line.iter().enumerate() {
            if char == '*' {
                let mut count = 0;
                let mut ratio = 1;
                for number in numbers.iter_mut() {
                    if number.is_near(line_i, char_i) {
                        count += 1;
                        ratio *= number.num;
                    }
                }

                if count == 2 {
                    res += ratio;
                }
            }
        }
    }

    res
}


fn get_numbers(lines: &[Vec<char>]) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (line_i, line) in lines.iter().enumerate() {
        let mut it = line.iter().enumerate();
        let mut current = String::new();
        let mut start = 0;
        let mut end = 0;
        loop {
            let next = it.next();
            match next {
                Some((i, &char @ '0'..='9')) => {
                    if current.len() == 0 {
                        start = i;
                    }
                    end = i;

                    current.push(char);
                },
                _ => {
                    if current.len() > 0 {
                        numbers.push(Number {
                            num: current.parse().unwrap(),
                            row: line_i,
                            start,
                            end: end + 1,
                            include: false,
                        });
                        current.clear();
                    }
                },
            }

            if next == None {
                break;
            }
        }
    }

    numbers
}
