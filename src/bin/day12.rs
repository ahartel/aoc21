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

fn traverse_graph_one_small_cave_twice(graph: &HashMap<String, Node>, cur_node: &Node, mut path: String, paths: &mut Vec<String>, small_cave_visited: bool) {
    let is_uppercase = cur_node.name.chars().nth(0).unwrap().is_uppercase();
    let is_start_end = 
    if is_uppercase || !small_cave_visited || !path.contains(&cur_node.name) {
        let visited_small_twice = !is_uppercase && path.contains(&cur_node.name);
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

    let lines = include_str!("../../data/day12/test.txt").lines();
    for line in lines {
        let mut nodes = line.split("-");
        let nodeA = String::from(nodes.next().unwrap());
        let nodeB = String::from(nodes.next().unwrap());
        if !graph.contains_key(&nodeA) {
            graph.insert(nodeA.clone(), Node{name: nodeA.clone(), children:vec![nodeB.clone(); 1]});
        }
        else {
            graph.get_mut(&nodeA).unwrap().children.push(nodeB.clone());
        }
        if !graph.contains_key(&nodeB) {
            graph.insert(nodeB.clone(), Node{name: nodeB.clone(), children:vec![nodeA; 1]});
        }
        else {
            graph.get_mut(&nodeB).unwrap().children.push(nodeA);
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
