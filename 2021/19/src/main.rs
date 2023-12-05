use std::cmp::max;
use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let input = get_input();

    let (res,scanner_poses) = solve1(&input);
    println!("{}", res);

    let res = solve2(&scanner_poses);
    println!("{}", res);
}

fn solve1(scanners: &[Vec<[i32;3]>]) -> (i32,Vec<[i32;3]>) {
    // Assume the first scanner has fixed coordinates and orientation
    let first = &scanners[0];
    let mut scanner_poses = Vec::new();
    scanner_poses.push([0,0,0]);
    let mut maps = Vec::new();
    let map: HashSet<[i32;3]> = first.iter().map(|pos| *pos).collect();
    maps.push(map);

    let rotation_count = 24;
    let mut scanners: Vec<_> = scanners.iter().skip(1).collect();
    while scanners.len() > 0 {
        let mut remove_i = None;
        'outer: for (scanner_i,scanner) in scanners.iter().enumerate() {
            let all_rotations: Vec<_> = scanner.iter().map(|pos| get_rotations(pos))
                .collect();

            for pos_index in 0..all_rotations.len() {
                for rot_index in 0..rotation_count {
                    for (map_i,map) in maps.iter().enumerate() {
                        for done_pos in map.iter() {
                            let offset = sub(*done_pos, all_rotations[pos_index][rot_index]);
                            let mut matching = 0;
                            for i in 0..all_rotations.len() {
                                let new_pos = add(offset, all_rotations[i][rot_index]);
                                if map.contains(&new_pos) {
                                    matching += 1;
                                }
                            }

                            if matching >= 12 {
                                scanner_poses.push(offset);
                                let mut new_map: HashSet<[i32;3]> = HashSet::new();
                                for i in 0..all_rotations.len() {
                                    let new_pos = add(offset, all_rotations[i][rot_index]);
                                    new_map.insert(new_pos);
                                }

                                println!("{} Matched with map {}", scanner_i, map_i);

                                maps.push(new_map);
                                remove_i = Some(scanner_i);

                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        let remove_i = remove_i.unwrap();
        scanners.remove(remove_i);
    }

    let mut full_map: HashSet<[i32;3]> = HashSet::new();
    for map in maps.iter() {
        full_map.extend(map);
    }

    (full_map.len() as i32, scanner_poses)
}

fn solve2(scanner_poses: &[[i32;3]]) -> i32 {
    let mut res = 0;
    for (i,pos1) in scanner_poses.iter().enumerate() {
        for pos2 in scanner_poses.iter().skip(i) {
            res = max(res, manhattan(*pos1, *pos2));
        }
    }

    res
}

fn manhattan(pos1: [i32;3], pos2: [i32;3]) -> i32 {
    let mut res = 0;
    for i in 0..3 {
        res += (pos1[i] - pos2[i]).abs()
    }
    res
}

fn add(pos1: [i32;3], pos2: [i32;3]) -> [i32;3] {
    let mut res = pos1;
    res[0] += pos2[0];
    res[1] += pos2[1];
    res[2] += pos2[2];
    res
}

fn sub(pos1: [i32;3], pos2: [i32;3]) -> [i32;3] {
    let mut res = pos1;
    res[0] -= pos2[0];
    res[1] -= pos2[1];
    res[2] -= pos2[2];
    res
}

fn get_rotations(pos: &[i32;3]) -> Vec<[i32;3]> {
    let mut res = Vec::new();
    let mut pos1 = *pos;
    // Euler angles
    for _ in 0..4 {
        // Upright
        res.push(pos1);

        // Upside down
        let mut pos2 = pos1;
        rotate_x(&mut pos2);
        rotate_x(&mut pos2);
        res.push(pos2);

        // Sideways
        let mut pos2 = pos1;
        rotate_x(&mut pos2);
        for _ in 0..4 {
            res.push(pos2);
            rotate_z(&mut pos2);
        }

        rotate_z(&mut pos1);
    }

    res
}

fn rotate_x(pos: &mut [i32;3]) {
    let old_pos = *pos;
    pos[0] = 1*old_pos[0] + 0*old_pos[1] + 0*old_pos[2];
    pos[1] = 0*old_pos[0] + 0*old_pos[1] + -1*old_pos[2];
    pos[2] = 0*old_pos[0] + 1*old_pos[1] + 0*old_pos[2];
}

fn rotate_z(pos: &mut [i32;3]) {
    let old_pos = *pos;
    pos[0] = 0*old_pos[0] + -1*old_pos[1] + 0*old_pos[2];
    pos[1] = 1*old_pos[0] + 0*old_pos[1] + 0*old_pos[2];
    pos[2] = 0*old_pos[0] + 0*old_pos[1] + 1*old_pos[2];
}

fn get_input() -> Vec<Vec<[i32;3]>> {
    let mut res = Vec::new();
    let stdin = stdin();
    loop {
        // Read line "--- scanner"
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut scanner = Vec::new();
        let done = loop {
            let mut input = String::new();
            if stdin.read_line(&mut input).unwrap() == 0 {
                break true;
            }

            let input = input.trim();
            if input.len() == 0 {
                break false;
            }

            let mut pos = input.split(',')
                .map(|s| s.parse::<i32>().unwrap());
            scanner.push([
                pos.next().unwrap(),
                pos.next().unwrap(),
                pos.next().unwrap(),
            ]);
        };
        res.push(scanner);

        if done {
            break;
        }
    }

    res
}
