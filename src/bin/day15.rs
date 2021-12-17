use std::collections::HashMap;


struct Node {
    children: Vec<(u32, usize, usize)>
}

fn traverse_graph_min_risk(graph: &HashMap<(usize, usize), Node>, min_risks: &mut HashMap<(usize, usize), u32>, cur_node: &Node, path_cost: u32) {
    // iterate sorted vector of children
    for child in &cur_node.children {
        let child_cost = child.0 + path_cost;
        let child_pos = &(child.1, child.2);
        if &child_cost < min_risks.get(child_pos).unwrap() {
            let min_risk = min_risks.get_mut(child_pos).unwrap();
            *min_risk = child_cost;
            traverse_graph_min_risk(graph, min_risks, graph.get(&(child.1, child.2)).unwrap(), child_cost);
        }
    }
}


fn main() {
    let mut graph : HashMap<(usize, usize), Node> = HashMap::new();
    let mut min_risks : HashMap<(usize, usize), u32> = HashMap::new();

    let lines = include_str!("../../data/day15/input.txt").lines();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for node in line.chars() {
            let risk : u32 = node.to_digit(10).unwrap();
            graph.insert((x, y), Node {children: Vec::new()});
            min_risks.insert((x, y), u32::MAX);
            if x > 0 {
                let parent_node = graph.get_mut(&(x-1, y)).unwrap();
                (*parent_node).children.push((risk, x, y));
            }
            if y > 0 {
                let parent_node = graph.get_mut(&(x, y-1)).unwrap();
                (*parent_node).children.push((risk, x, y));
                (*parent_node).children.sort_by(|a, b| a.0.cmp(&b.0));
            }
            x += 1;
        }
        y += 1;
    }
    // for (k, v) in &graph {
    //     println!("({}, {}) @ {} -> {:?}", k.0, k.1, v.risk, v.children)
    // }

    let start_node = graph.get(&(0, 0)).unwrap();
    let start_cost = 0;
    traverse_graph_min_risk(&graph, &mut min_risks, start_node, start_cost);
    println!("{}", min_risks.get(&(y-1, y-1)).expect("Risk to target not found"));
}
