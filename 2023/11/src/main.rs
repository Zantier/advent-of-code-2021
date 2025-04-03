use std::{
    cmp::max,
    collections::HashSet,
    io,
};

type Pos = [i64;2];

fn main() {
    let mut galaxies = Vec::new();
    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push([i as i64, j as i64]);
            }
        }
    }

    for gap_size in [2, 10, 100, 1000000] {
        let res = solve(&galaxies, gap_size);
        println!("{}", res);
    }
}


fn solve(galaxies: &[Pos], gap_size: i64) -> i64 {
    let mut rows_used: HashSet<i64> = HashSet::new();
    let mut cols_used: HashSet<i64> = HashSet::new();

    for &[i,j] in galaxies {
        rows_used.insert(i);
        cols_used.insert(j);
    }

    // Cumulative number of spaces
    let row_gaps = used_to_cumulative_gaps(&rows_used, gap_size);
    let col_gaps = used_to_cumulative_gaps(&cols_used, gap_size);

    let mut res = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i+1) {
            for j in 0..2 {
                res += (galaxy1[j] - galaxy2[j]).abs();
            }

            res += (row_gaps[galaxy1[0] as usize] - row_gaps[galaxy2[0] as usize]).abs();
            res += (col_gaps[galaxy1[1] as usize] - col_gaps[galaxy2[1] as usize]).abs();
        }
    }

    res
}


/// Convert from a set of used numbers to a cumulative list of how many
/// gaps in the numbers
fn used_to_cumulative_gaps(used: &HashSet<i64>, gap_size: i64) -> Vec<i64> {
    let mut len = 0;
    for &num in used {
        len = max(len, num + 1);
    }

    let mut gaps = 0;
    let mut res = Vec::new();
    for i in 0..len {
        if !used.contains(&i) {
            gaps += gap_size - 1;
        }

        res.push(gaps);
    }

    res
}
