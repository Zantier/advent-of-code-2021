use std::io;


type Pos = [i32;2];


trait MapVisitor {
    fn visit(&mut self, pos: &Pos, dir: i32, map: &Map);
}


/// Calculate whether the pipe is going anti-clockwise (1) or clockwise (3)
struct GetOrientation {
    /// The total of the direction changes
    total: i32,
    last_dir: Option<i32>,
}


impl GetOrientation {
    fn new() -> Self {
        Self {
            total: 0,
            last_dir: None,
        }
    }
}


impl MapVisitor for GetOrientation {
    fn visit(&mut self, _pos: &Pos, dir: i32, _map: &Map) {
        match self.last_dir {
            Some(last_dir) => {
                match (dir - last_dir + 4) % 4 {
                    1 => self.total += 1,
                    3 => self.total -= 1,
                    _ => (),
                }
            },
            None => (),
        }

        self.last_dir = Some(dir);
    }
}


/// Flood fill to find how much ground within the pipe loop
struct Flood1 {
    visited: Vec<Vec<bool>>,
}
impl Flood1 {
    fn new(size: Pos) -> Self {
        let visited = vec![vec![false;size[1] as usize];size[0] as usize];
        Self {
            visited,
        }
    }
}


impl MapVisitor for Flood1 {
    fn visit(&mut self, pos: &Pos, _dir: i32, _map: &Map) {
        self.visited[pos[0] as usize][pos[1] as usize] = true;
    }
}


struct Flood2 {
    /// The orientation of the pipe. 1 = anti-clockwise, 3 = clockwise
    dir: i32,
    visited: Vec<Vec<bool>>,
    result: i32,
}
impl Flood2 {
    fn new(visited: Vec<Vec<bool>>, dir: i32) -> Self {
        Self {
            dir,
            visited,
            result: 0,
        }
    }
}


impl MapVisitor for Flood2 {
    fn visit(&mut self, pos: &Pos, dir: i32, map: &Map) {
        let dir = (dir + self.dir) % 4;
        let dir_vec = num_to_vec(dir);
        let pos = add_vec(*pos, dir_vec);
        let mut poses = vec![pos];

        loop {
            let pos = match poses.pop() {
                Some(pos) => pos,
                None => break,
            };

            if !map.in_bounds(pos) {
                continue;
            }
            if self.visited[pos[0] as usize][pos[1] as usize] {
                continue;
            }

            self.visited[pos[0] as usize][pos[1] as usize] = true;
            self.result += 1;

            for dir in 0..3 {
                let dir_vec = num_to_vec(dir);
                let pos = add_vec(pos, dir_vec);
                poses.push(pos);
            }
        }
    }
}


struct Map {
    lines: Vec<Vec<i32>>,
    size: Pos,
}


impl Map {
    fn new(lines: &[String]) -> Self {
        let lines: Vec<_> = lines.iter()
            .map(|line| parse_line(line))
            .collect();
        let size = [lines.len() as i32, lines[0].len() as i32];

        Map {
            lines,
            size,
        }
    }


    fn get_start(&self) -> Pos {
        for (i, line) in self.lines.iter().enumerate() {
            for (j, &x) in line.iter().enumerate() {
                if x == 16 {
                    return [i as i32,j as i32]
                }
            }
        }

        [0,0]
    }


    fn in_bounds(&self, pos: Pos) -> bool {
        pos[0] >= 0 && pos[0] < self.size[0] && pos[1] >= 0 && pos[1] < self.size[1]
    }


    fn get(&self, pos: Pos) -> i32 {
        if !self.in_bounds(pos) {
            return 0;
        }

        self.lines[pos[0] as usize][pos[1] as usize]
    }


    fn traverse(&self, mut visitor: Option<&mut impl MapVisitor>) -> i32 {
        let start = self.get_start();
        let mut pos = start;
        let mut backwards = 0;
        for dir in 0..4 {
            let dir_vec = num_to_vec(dir);
            let new_pos = add_vec(pos, dir_vec);
            let opposite_dir = get_opposite_dir(dir);

            if self.get(new_pos) & 1 << opposite_dir > 0 {
                if let Some(visitor) = visitor.as_mut() {
                    visitor.visit(&pos, dir, self);
                }
                pos = new_pos;
                backwards = opposite_dir;
                break;
            }
        }

        let mut length = 0;
        while pos != start {
            length += 1;
            let bits = self.get(pos);
            for dir in 0..4 {
                if dir != backwards && bits & 1 << dir > 0 {
                    if let Some(visitor) = visitor.as_mut() {
                        visitor.visit(&pos, dir, self);
                    }
                    let dir_vec = num_to_vec(dir);
                    pos = add_vec(pos, dir_vec);
                    backwards = get_opposite_dir(dir);
                    break;
                }
            }
        }

        (length + 1) / 2
    }
}


fn main() {
    let lines: Vec<_> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let map = Map::new(&lines);

    let res = solve1(&map);
    println!("{}", res);

    let res = solve2(&map);
    println!("{}", res);
}


fn solve1(map: &Map) -> i32 {
    let none: Option<&mut GetOrientation> = None;
    let res = map.traverse(none);
    res
}


fn solve2(map: &Map) -> i32 {
    let mut get_orientation = GetOrientation::new();
    map.traverse(Some(&mut get_orientation));

    let dir = if get_orientation.total > 0 { 1 } else { 3 };

    let mut flood1 = Flood1::new(map.size);
    map.traverse(Some(&mut flood1));
    let mut flood2 = Flood2::new(flood1.visited, dir);
    map.traverse(Some(&mut flood2));

    flood2.result
}


fn get_opposite_dir(dir: i32) -> i32 {
    (dir + 2) % 4
}


fn parse_line(line: &str) -> Vec<i32> {
    line.chars()
        .map(parse_char)
        .collect()
}


/// Parse map char into bitset of cardinal directions
fn parse_char(ch: char) -> i32 {
    match ch {
        '|' => return 10,
        '-' => return 5,
        'L' => return 3,
        'J' => return 6,
        '7' => return 12,
        'F' => return 9,
        '.' => return 0,
        'S' => return 16,
        _ => panic!("bad map character"),
    }
}


/// Convert number representing pipe directions to an (i,j) directional vector
fn num_to_vec(num: i32) -> Pos {
    match num {
        0 => return [0,1],
        1 => return [-1,0],
        2 => return [0,-1],
        3 => return [1,0],
        _ => panic!("bad numeric direction"),
    }
}


fn add_vec(vec1: Pos, vec2: Pos) -> Pos {
    [vec1[0] + vec2[0], vec1[1] + vec2[1]]
}
