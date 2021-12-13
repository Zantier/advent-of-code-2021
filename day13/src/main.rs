use std::collections::HashSet;
use std::io::stdin;

#[derive(Clone,Copy,Debug)]
struct Fold {
    axis: usize,
    pos: i32,
}

fn main() {
    let stdin = stdin();
    let mut points = HashSet::new();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "" {
            break;
        }

        let input: Vec<_> = input.split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        points.insert([input[0],input[1]]);
    }

    let mut folds = Vec::new();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input: Vec<_> = input.split_whitespace().collect();
        let input: Vec<_> = input[2].split('=').collect();
        let axis = match input[0] {
            "x" => 0,
            _ => 1,
        };
        folds.push(Fold { axis, pos: input[1].parse::<i32>().unwrap() });
    }

    let res = solve1(&points, folds[0]);
    println!("{}", res);
}

fn solve1(points: &HashSet<[i32;2]>, fold: Fold) -> i32 {
    let mut points = points.clone();

    do_fold(&mut points, fold);

    points.len() as i32
}

fn do_fold(points: &mut HashSet<[i32;2]>, fold: Fold) {
    let mut old_points = Vec::new();
    for &point in points.iter() {
        if point[fold.axis] >= fold.pos {
            old_points.push(point);
        }
    }

    for point in old_points.iter() {
        points.remove(point);
    }

    for point in old_points.iter_mut() {
        if point[fold.axis] > fold.pos {
            point[fold.axis] = 2*fold.pos - point[fold.axis];
            points.insert(*point);
        }
    }
}
