use std::io;
use std::io::BufRead;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    id: i32,
    out_nodes: HashSet<i32>,
    // in_nodes: HashSet<i32>,
}

fn new_node(id: i32) -> Node {
    Node {
        id: id,
        out_nodes: HashSet::new(),
    }
}

fn _connect(graph: &mut HashMap<i32, Node>, id: i32, dst: i32) {
    // println!("Connecting {} -> {}", id, dst);
    let node = graph.entry(id).or_insert(new_node(id));
    node.out_nodes.insert(dst);
}

fn add_edge(graph: &mut HashMap<i32, Node>, src: i32, dst: i32) {
    _connect(graph, src, dst);
    _connect(graph, dst, src);
}

fn connect_component(graph: &mut HashMap<i32, Node>, visited: &mut Vec<bool>, src: i32) -> i32 {
    let mut component_nodes = 0;
    let mut queue: LinkedList<i32> = LinkedList::new();

    component_nodes += 1;
    visited[src as usize] = true;
    queue.push_back(src);
    while !queue.is_empty() {
        let src = queue.pop_front().unwrap();
        for dst in &graph.get(&src).unwrap().out_nodes {
            if !visited[*dst as usize] {
                // println!("{} -> {}", src, dst);
                component_nodes += 1;
                visited[*dst as usize] = true;
                queue.push_back(*dst);
            }
        }
    }
    component_nodes
}

fn parse_line(graph: &mut HashMap<i32, Node>, line: &str) {
    let tokens: Vec<&str> = line.split(" <-> ").collect();
    let src = tokens[0].parse().unwrap();
    for dst in tokens[1].split(", ") {
        add_edge(graph, src, dst.parse().unwrap());
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| it.len() > 0)
        .collect();

    let mut graph = HashMap::new();
    for line in lines {
        parse_line(&mut graph, &line);
    }
    let mut visited = Vec::new();
    for i in 0..graph.len() {
        visited.push(false);
    }

    let mut root = 0;
    let mut components = 0;
    while root != -1 {
        components += 1;
        let component_nodes = connect_component(&mut graph, &mut visited, root);
        println!("Partition rooted at {} has {} nodes", root, component_nodes);

        root = -1;
        for (id, b) in visited.iter().enumerate() {
            if !b {
                root = id as i32;
                break;
            }
        }
    }

    println!("Found {} partitions", components);
}
