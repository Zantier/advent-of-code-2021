use regex::Regex;
use std::{
    collections::HashMap,
    io,
};


struct NodeParser {
    re: Regex,
}


impl NodeParser {
    fn new() -> Self {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        NodeParser {
            re,
        }
    }


    fn parse_node(&self, line: &str) -> Vec<String> {
        let captures = self.re.captures(line).unwrap();
        let mut iter = captures.iter();
        iter.next();
        let strs: Vec<_> = iter
            .map(|mat| mat.unwrap().as_str().to_string())
            .collect();
        strs
    }
}


fn main() {
    let mut lines = io::stdin().lines()
        .map(|line| line.unwrap());

    let dirs: Vec<_> = lines.next().unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 1 } else { 2 })
        .collect();
    lines.next();

    let parser = NodeParser::new();
    let nodes: Vec<_> = lines.map(|line| parser.parse_node(&line))
        .collect();

    let res = solve1(&dirs, &nodes);
    println!("{}", res);

    let res = solve2(&dirs, &nodes);
    println!("{}", res);
}


fn solve1(dirs: &[usize], nodes: &[Vec<String>]) -> i32 {
    let mut map: HashMap<&str, _> = HashMap::new();
    for node in nodes {
        map.insert(&node[0], node);
    }

    let mut node_name = "AAA";
    let mut res = 0;
    let mut dir_index = 0;
    while node_name != "ZZZ" {
        res += 1;
        let node = match map.get(node_name) {
            Some(node) => node,
            None => return 0,
        };

        let dir = dirs[dir_index];
        node_name = &node[dir];
        dir_index = (dir_index + 1) % dirs.len();
    }

    res
}


fn solve2(dirs: &[usize], nodes: &[Vec<String>]) -> i32 {
    let mut map: HashMap<&str, _> = HashMap::new();
    for node in nodes {
        map.insert(&node[0], node);
    }

    let mut node_names: Vec<&str> = Vec::new();
    for node in nodes {
        if node[0].ends_with('A') {
            node_names.push(&node[0]);
        }
    }

    let mut res = 0;
    let mut dir_index = 0;
    while !solve2_finished(&node_names) {
        res += 1;
        if res % 100000 == 0 {
            println!("{}", res);
        }
        let dir = dirs[dir_index];

        for node_name in node_names.iter_mut() {
            let node = match map.get(node_name) {
                Some(node) => node,
                None => return 0,
            };
            *node_name = &node[dir];
        }


        dir_index = (dir_index + 1) % dirs.len();
    }

    res
}


fn solve2_finished(node_names: &[&str]) -> bool {
    node_names.iter().all(|name| name.ends_with('Z'))
}
