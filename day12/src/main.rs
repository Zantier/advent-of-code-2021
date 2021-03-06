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

    let res = Solve2::solve(&nodes, &graph);
    println!("{}", res);
}

struct Solve1 {
    graph: HashMap<String,HashSet<String>>,
}

impl Solve1 {
    fn solve(nodes: &HashSet<String>,
        graph: &HashMap<String,HashSet<String>>) -> i32 {
        let data = Solve1 {
            graph: graph.clone(),
        };
        let mut visited = HashMap::from_iter(nodes.iter()
            .map(|s| (s.to_string(), false)));

        data.dfs("start", &mut visited)
    }

    fn dfs(&self, node: &str, visited: &mut HashMap<String,bool>) -> i32 {
        if node == "end" {
            return 1;
        }
        if visited[node] {
            return 0;
        }

        if node.chars().next().unwrap().is_ascii_lowercase() {
            visited.insert(node.to_string(), true);
        }
        let mut res = 0;
        for node2 in self.graph[node].iter() {
            res += self.dfs(node2, visited);
        }
        visited.insert(node.to_string(), false);

        res
    }
}

struct Solve2 {
    graph: HashMap<String,HashSet<String>>,
}
struct Solve2Mut {
    visited: HashMap<String,i32>,
    visited_twice: bool,
}

impl Solve2 {
    fn solve(nodes: &HashSet<String>,
        graph: &HashMap<String,HashSet<String>>) -> i32 {
        let data = Solve2 {
            graph: graph.clone(),
        };
        let mut data_mut = Solve2Mut {
            visited: HashMap::from_iter(nodes.iter()
                .map(|s| (s.to_string(), 0))),
            visited_twice: false,
        };

        data.dfs("start", &mut data_mut)
    }

    fn dfs(&self, node: &str, data_mut: &mut Solve2Mut) -> i32 {
        if node == "end" {
            return 1;
        }
        if node == "start" && data_mut.visited[node] == 1 {
            return 0;
        }
        if data_mut.visited[node] > 0 && data_mut.visited_twice {
            return 0;
        }


        if data_mut.visited[node] > 0 {
            data_mut.visited_twice = true;
        }
        if node.chars().next().unwrap().is_ascii_lowercase() {
            *data_mut.visited.entry(node.to_string()).or_default() += 1;
        }
        let mut res = 0;
        for node2 in self.graph[node].iter() {
            res += self.dfs(node2, data_mut);
        }

        if data_mut.visited[node] > 1 {
            data_mut.visited_twice = false;
        }
        *data_mut.visited.entry(node.to_string()).or_default() -= 1;

        res
    }
}
