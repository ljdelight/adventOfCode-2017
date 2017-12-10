use std::io;
use std::io::BufRead;

struct Node {
    id: i32,
    name: String,
    out: Vec<String>,
}

fn parse_line(line: &String) -> Node {
    let name = &line[0..line.find(' ').unwrap()];
    let idStart = 1 + line.find('(').unwrap();
    let idEnd = line.find(')').unwrap();
    let id: i32 = line[idStart..idEnd].parse().unwrap();

    let mut neighbors: Vec<String> = Vec::new();
    if line.contains("->") {
        let vecStart = line.find("->").unwrap() + 3;
        let strs: Vec<&str> = line[vecStart..].split(", ").collect();
        for s in &strs {
            neighbors.push(s.to_string());
        }
    }
    // println!("Creating node '{}' '{}' {:?}", id, name, neighbors);
    Node {
        id: id,
        name: String::from(name),
        out: neighbors,
    }
}


fn weight(graph: &Vec<Node>, node: &Node) -> i32 {
    let mut w = node.id;
    let mut bal = -1;

    let mut childWeights = Vec::new();
    for neighbor in &node.out {
        let idx = graph.iter().position(|it| it.name.eq(neighbor)).unwrap();
        let childW = weight(&graph, &graph[idx]);
        childWeights.push(childW);
        w += childW;
    }
    let mut allEqual = true;
    for it in &childWeights {
        if *it != childWeights[0] {
            allEqual = false;
        }
    }
    if !allEqual {
        println!(
            "Node {} has a not-matching child: {:?}",
            node.name,
            childWeights
        );
    }
    w
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

    let mut graph = Vec::new();
    for line in &lines {
        let node = parse_line(&line);
        graph.push(node);
    }

    let mut allNodes: Vec<&String> = graph.iter().map(|it| &it.name).collect();
    for node in &graph {
        for neighbor in &node.out {
            match allNodes.iter().position(|it| *it == neighbor) {
                Some(idx) => {
                    allNodes.remove(idx);
                }
                _ => {
                    println!("Not found");
                }
            }
        }
    }
    let baseNodeName = allNodes[0];
    let baseNode = &graph[graph
                              .iter()
                              .position(|it| it.name.eq(baseNodeName))
                              .unwrap()];
    println!(
        "Base node is: {:?} with weight {}",
        baseNode.name,
        weight(&graph, &baseNode)
    );

    for neighbor in &baseNode.out {
        let n = &graph[graph.iter().position(|it| it.name.eq(neighbor)).unwrap()];
        println!(
            "Node {} (w={}) has tree weight of {}",
            neighbor,
            n.id,
            weight(&graph, &n)
        );
    }
    // graph.iter().position(|it| it.name.eq( neighbor)).unwrap()
    // println!("Weight of root is {}", weight(&graph, &graph[0]));
}
