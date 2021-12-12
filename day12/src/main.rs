use std::collections::{HashMap,HashSet};
use std::io::stdin;
use std::iter::FromIterator;

fn main() {
    let stdin = stdin();
    let mut nodes: HashSet<String> = HashSet::new();
    let mut graph: HashMap<String,HashSet<String>> = HashMap::new();
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }

        let line: Vec<_> = line.trim().split('-').collect();
        nodes.insert(line[0].to_string());
        nodes.insert(line[1].to_string());
        let node1 = graph.entry(line[0].to_string())
            .or_insert(HashSet::new());
        node1.insert(line[1].to_string());
        let node2 = graph.entry(line[1].to_string())
            .or_insert(HashSet::new());
        node2.insert(line[0].to_string());
    }

    let res = Solve1::solve(&nodes, &graph);
    println!("{}", res);
}

struct Solve1 {
    graph: HashMap<String,HashSet<String>>,
    visited: HashMap<String,bool>,
}

impl Solve1 {
    fn solve(nodes: &HashSet<String>,
        graph: &HashMap<String,HashSet<String>>) -> i32 {
        let mut data = Solve1 {
            graph: graph.clone(),
            visited: HashMap::from_iter(nodes.iter()
                .map(|s| (s.to_string(), false))),
        };

        data.dfs("start")
    }

    fn dfs(&mut self, node: &str) -> i32 {
        if node == "end" {
            return 1;
        }
        if self.visited[node] {
            return 0;
        }

        if node.chars().next().unwrap().is_ascii_lowercase() {
            self.visited.insert(node.to_string(), true);
        }
        let mut res = 0;
        for node2 in self.graph.entry(node.to_string()).or_default().iter() {
            res += self.dfs(node2);
        }
        self.visited.insert(node.to_string(), false);

        res
    }
}
