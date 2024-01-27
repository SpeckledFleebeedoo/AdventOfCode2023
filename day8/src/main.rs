use core::panic;

#[derive(Clone)]
struct Node {
    name: String,
    left: String,
    right: String
}

fn find_node(goal: &str, nodes: &Vec<Node>) -> Option<Node> {
    for node in nodes {
        if node.name == goal {
            return Some(node.to_owned());
        }
    }
    return None;
}

fn find_start_nodes(nodes: &Vec<Node>) -> Vec<Node> {
    let mut start_nodes: Vec<Node> = Vec::new();
    for node in nodes {
        if node.name.chars().collect::<Vec<char>>()[2] == 'A' {
            start_nodes.push(node.clone())
        }
    }
    return start_nodes;
}

fn part1() {
    let file_path = "input.txt";
    let input = std::fs::read_to_string(file_path).expect("Could not open file");

    let lines: Vec<&str> = input.split("\n").collect();
    let instructions = lines[0].to_owned();
    let nodes_str_vec = lines[2..].to_owned();

    let mut nodes: Vec<Node> = Vec::new();
    for node_str in nodes_str_vec {
        // VJN = (LNC, RRK)
        let node_split: Vec<&str> = node_str.split_whitespace().collect();
        nodes.push(Node{name: node_split[0].to_owned(), left: node_split[2].trim_end_matches(',').trim_start_matches('(').to_owned(), right: node_split[3].trim_end_matches(')').to_owned()});
    }

    let start_node_name = "AAA";
    let end_node_name = "ZZZ";

    // let current_nodes: Vec<Node> = find_start_nodes(&nodes);


    let mut current_node = find_node(start_node_name, &nodes).unwrap();
    let mut total_steps = 0;

    let mut end_found = false;

    while !end_found {
        for c in instructions.chars() {
            let next_node: &String = match c {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => panic!("Invalid character found"),
            };
            total_steps += 1;

            if next_node == end_node_name {
                end_found = true;
                break;
            } else {
                println!("{}", next_node);
                current_node = find_node(&next_node, &nodes).unwrap();
            }
        }
    }
    println!("{}", total_steps)



}


fn main() {
    part1();
}