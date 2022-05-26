use std::collections::HashMap;


struct Node {
    name: String,
    children: Vec<String>
}

fn traverse_graph_all_small_caves_once(graph: &HashMap<String, Node>, cur_node: &Node, mut path: String, paths: &mut Vec<String>) {
    if cur_node.name.chars().nth(0).unwrap().is_uppercase() || !path.contains(&cur_node.name) {
        path.push_str(&cur_node.name);
        path.push_str(",");

        for child in &cur_node.children {
            if child != "end" {
                let child_node = graph.get(child).unwrap();
                traverse_graph_all_small_caves_once(graph, child_node, path.clone(), paths);
            }
            else {
                path.push_str(&child);
                //println!("{}", path);
                paths.push(path.clone());
            }
        }
    }
}

fn traverse_graph_one_small_cave_twice(
    graph: &HashMap<String, Node>,
    cur_node: &Node,
    mut path: String,
    paths: &mut Vec<String>,
    small_cave_twice: bool)
{
    let is_uppercase = cur_node.name.chars().nth(0).unwrap().is_uppercase();
    let is_start_end = cur_node.name == "start" || cur_node.name == "end";
    if is_uppercase || (!small_cave_twice && !is_start_end) || !path.contains(&cur_node.name) {
        let visited_small_twice = small_cave_twice || (!is_uppercase && !is_start_end && path.contains(&cur_node.name));
        path.push_str(&cur_node.name);
        path.push_str(",");

        for child in &cur_node.children {
            if child != "end" {
                let child_node = graph.get(child).unwrap();
                traverse_graph_one_small_cave_twice(graph, child_node, path.clone(), paths, visited_small_twice);
            }
            else {
                path.push_str(&child);
                //println!("{}", path);
                paths.push(path.clone());
            }
        }
    }
}

fn main() {
    let mut graph : HashMap<String, Node> = HashMap::new();

    let lines = include_str!("../../data/day12/input.txt").lines();
    for line in lines {
        let mut nodes = line.split("-");
        let node_a = String::from(nodes.next().unwrap());
        let node_b = String::from(nodes.next().unwrap());
        if !graph.contains_key(&node_a) {
            graph.insert(node_a.clone(), Node{name: node_a.clone(), children:vec![node_b.clone(); 1]});
        }
        else {
            graph.get_mut(&node_a).unwrap().children.push(node_b.clone());
        }
        if !graph.contains_key(&node_b) {
            graph.insert(node_b.clone(), Node{name: node_b.clone(), children:vec![node_a; 1]});
        }
        else {
            graph.get_mut(&node_b).unwrap().children.push(node_a);
        }
    }

    let start_string = String::from("start");
    let start_node = graph.get(&start_string).unwrap();
    {
        let mut paths : Vec<String> = Vec::new();
        let path = String::from("");
        traverse_graph_all_small_caves_once(&graph, &start_node, path.clone(), &mut paths);
        println!("Found {} paths", paths.len());
    }
    {
        let mut paths : Vec<String> = Vec::new();
        let path = String::from("");
        traverse_graph_one_small_cave_twice(&graph, &start_node, path.clone(), &mut paths, false);
        println!("Found {} paths", paths.len());
    }
}
