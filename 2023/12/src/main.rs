use std::{
    io,
    str::FromStr,
};


#[derive(Clone)]
struct Row {
    row: Vec<Option<bool>>,
    groups: Vec<i32>,
}


impl Row {
    fn count_possibilities(&mut self, index: usize) -> i32 {
        if index == self.row.len() {
            let mut damage_count = 0;
            let mut group_index = 0;
            for spring in &self.row {
                if let Some(true) = spring {
                    damage_count += 1;
                } else if damage_count > 0 {
                    if group_index >= self.groups.len() {
                        return 0;
                    }

                    if self.groups[group_index] != damage_count {
                        return 0;
                    }

                    group_index += 1;
                    damage_count = 0;
                }
            }

            if damage_count > 0 {
                if group_index >= self.groups.len() {
                    return 0;
                }

                if self.groups[group_index] != damage_count {
                    return 0;
                }

                group_index += 1;
            }

            if group_index < self.groups.len() {
                return 0;
            }

            return 1;
        }

        match self.row[index] {
            Some(_) => return self.count_possibilities(index + 1),
            _ => (),
        }

        let mut res = 0;
        self.row[index] = Some(true);
        res += self.count_possibilities(index + 1);
        self.row[index] = Some(false);
        res += self.count_possibilities(index + 1);
        self.row[index] = None;

        res
    }
}


impl FromStr for Row {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split_whitespace();
        let row = split.next().unwrap()
            .chars()
            .map(char_to_damaged)
            .collect();
        let groups = split.next().unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Row {
            row,
            groups,
        })
    }
}


fn main() {
    let rows: Vec<Row> = io::stdin().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let res = solve1(&rows);
    println!("{}", res);

    let rows: Vec<_> = rows.iter()
        .map(|row| make_bigger_row(row))
        .collect();

    let res = solve2(&rows);
    println!("{}", res);
}


fn solve1(rows: &[Row]) -> i32 {
    let mut rows = rows.to_vec();
    rows.iter_mut()
        .map(|row| row.count_possibilities(0))
        .sum()
}


fn solve2(rows: &[Row]) -> i32 {
    let mut rows = rows.to_vec();
    rows.iter_mut()
        .map(|row| row.count_possibilities(0))
        .sum()
}


fn char_to_damaged(ch: char) -> Option<bool> {
    match ch {
        '#' => Some(true),
        '.' => Some(false),
        '?' => None,
        _ => panic!("Not valid char: {}", ch),
    }
}


fn make_bigger_row(row_param: &Row) -> Row {
    let mut row = Vec::new();
    let mut groups = Vec::new();
    for i in 0..5 {
        if i > 0 {
            row.push(None);
        }
        row.extend(&row_param.row);
        groups.extend(&row_param.groups);
    }

    Row {
        row,
        groups,
    }
}
