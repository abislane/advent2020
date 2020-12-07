use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct BagNode {
    node_type: String,
    edges: HashSet<BagEdge>, 
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
struct BagEdge {
    edge_type: String,
    count: i64,
}

fn parse_bag_node(line: &str) -> BagNode {
    let mut parts = line.split(" bags contain ");
    let node_type = parts.next().unwrap().to_string();
    let children = parts.next().unwrap();
    let mut edges = HashSet::new();

    let bag_regex = Regex::new(r"([0-9]+) ([a-z ]+) bags?.?").unwrap();
    if children.starts_with("no") {
        return BagNode { node_type: node_type, edges: HashSet::new() }
    }
    for child in children.split(", ") {
        let captures = bag_regex.captures(child).unwrap();
        edges.insert(BagEdge{
            edge_type: captures.get(2).unwrap().as_str().to_string(),
            count: captures.get(1).unwrap().as_str().parse::<i64>().unwrap()
        });
    }

    BagNode{ node_type, edges }
}

fn parse_bags(contents: &str) -> HashMap<String, HashSet<BagEdge>> {
    let mut result = HashMap::new();

    for line in contents.lines() {
        let node = parse_bag_node(line);
        result.insert(node.node_type, node.edges);
    }

    result
}

fn part1(bags: &HashMap<String, HashSet<BagEdge>>) -> usize {
    // first, create the unweighted graph needed for this problem
    let mut back_graph = HashMap::new();
    for (bag, edges) in bags {
        for edge in edges {
            let other_bag = edge.edge_type.clone();
            if !back_graph.contains_key(&other_bag) {
                back_graph.insert(other_bag.clone(), Vec::new());
            }
            back_graph.get_mut(&other_bag).unwrap().push(bag);
        }
    }

    // next, traverse the graph from the root
    let mut queue = vec!["shiny gold".to_string()];
    let mut visited = HashSet::new();
    while queue.len() > 0 {
        let top = queue.remove(0);
        if visited.contains(&top) {
            continue;
        }
        visited.insert(top.clone());
        if !back_graph.contains_key(&top) {
            continue;
        }
        for &neighbor in back_graph.get(&top).unwrap() {
            if !visited.contains(neighbor) {
                queue.push(neighbor.clone());
            }
        }
    }

    // return the number of visited nodes
    visited.len() - 1
}

fn total_bags(bag_type: String, bags: &HashMap<String, HashSet<BagEdge>>) -> i64 {
    let mut result = 1;
    for edge in bags.get(&bag_type).unwrap() {
        result += edge.count * total_bags(edge.edge_type.clone(), bags);
    }
    result
}

fn part2(bags: &HashMap<String, HashSet<BagEdge>>) -> i64 {
    total_bags("shiny gold".to_string(), bags) - 1
}

fn main() {
    let mut args = env::args();
    // Skip first arg, it's just the program name
    args.next();
    let file_name = args.next().unwrap();
    let contents = fs::read_to_string(file_name).unwrap();

    let bags = parse_bags(&contents);

    let result1 = part1(&bags);
    println!("{}", result1);

    let result2 = part2(&bags);
    println!("{}", result2);
}
