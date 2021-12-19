use std::collections::BinaryHeap;
use std::cmp::Ordering;


#[derive(Clone)]
struct Node {
    risk: u32,
    pos: (usize, usize),
    children: Vec<(u32, usize, usize)>
}

fn traverse_graph_min_risk(
    graph: &Vec<Vec<Node>>,
    min_risks: &mut Vec<Vec<u32>>,
    cur_node: &Node,
    path_cost: u32)
{
    // iterate sorted vector of children
    for child in &cur_node.children {
        let child_cost = child.0 + path_cost;
        if child_cost < min_risks[child.2][child.1] {
            min_risks[child.2][child.1] = child_cost;
            traverse_graph_min_risk(graph, min_risks, &graph[child.2][child.1], child_cost);
        }
    }
    // if cur_node.children.len() == 0 || shortest_path.len() > 0 {
    //     if cur_node.children.len() == 0 {
    //         shortest_path.clear();
    //     }
    //     shortest_path.push(cur_node.pos);
    // }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Node>>, start: (usize, usize), goal: (usize, usize), array_size: usize) -> Option<u32> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; array_size]; array_size];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.1][start.0] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position.1][position.0].children {
            let next = State { cost: cost + edge.0, position: (edge.1, edge.2) };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1][next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1][next.position.0] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn solve_part_1() {
    let lines = include_str!("../../data/day15/input.txt").lines();
    let array_size = lines.clone().count();
    let mut graph : Vec<Vec<Node>> = vec![vec![Node{risk:0, pos: (0, 0), children: Vec::new()}; array_size]; array_size];
    let mut min_risks : Vec<Vec<u32>> = vec![vec![u32::MAX; array_size]; array_size];

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for node in line.chars() {
            let risk : u32 = node.to_digit(10).unwrap();
            graph[y][x].risk = risk;
            graph[y][x].pos = (x, y);
            if x > 0 {
                graph[y][x-1].children.push((risk, x, y));
            }
            if y > 0 {
                graph[y-1][x].children.push((risk, x, y));
                graph[y-1][x].children.sort_by(|a, b| a.0.cmp(&b.0));
            }
            x += 1;
        }
        y += 1;
    }
    // for (k, v) in &graph {
    //     println!("({}, {}) @ {} -> {:?}", k.0, k.1, v.risk, v.children)
    // }

    let start_node = &graph[0][0];
    let start_cost = 0;
    //let mut shortest_path = Vec::new();
    traverse_graph_min_risk(&graph, &mut min_risks, start_node, start_cost);
    println!("{}", min_risks[y-1][y-1]);
    //println!("{:?}", shortest_path);
    let real_cost = shortest_path(&graph, (0, 0), (array_size-1, array_size-1), array_size);
    println!("Real cost: {}", real_cost.unwrap());
}


fn solve_part_2() {
    let lines = include_str!("../../data/day15/input.txt").lines();
    let array_size = lines.clone().count() * 5;

    let mut graph : Vec<Vec<Node>> = vec![vec![Node{risk:0, pos: (0, 0), children: Vec::new()}; array_size]; array_size];
    let mut min_risks : Vec<Vec<u32>> = vec![vec![u32::MAX; array_size]; array_size];

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for node in line.chars() {
            let width = line.len();
            let risk : u32 = node.to_digit(10).unwrap();
            for x_offset in 0..5 {
                for y_offset in 0..5 {
                    let real_x = x + x_offset*width;
                    let real_y = y + y_offset*width;
                    let raw_added_risk = usize::try_from(risk).unwrap() + x_offset + y_offset;
                    if raw_added_risk >= 19 {
                        println!("High risk detected: {}", raw_added_risk);
                    }
                    let cur_risk_usize = (raw_added_risk)%10;
                    let mut cur_risk = cur_risk_usize.try_into().unwrap();
                    if raw_added_risk > 9 {
                        cur_risk += 1;
                    }
                    graph[real_y][real_x].risk = cur_risk;
                    graph[real_y][real_x].pos = (real_x, real_y);
                }
            }
            x += 1;
        }
        y += 1;
    }
    
    for real_y in 0..y*5 {
        for real_x in 0..y*5 {
            let this_risk = graph[real_y][real_x].risk;
            if real_x > 0 {
                graph[real_y][real_x-1].children.push((this_risk, real_x, real_y));
            }
            if real_x < array_size-1 {
                graph[real_y][real_x+1].children.push((this_risk, real_x, real_y));
            }
            if real_y > 0 {
                graph[real_y-1][real_x].children.push((this_risk, real_x, real_y));
                graph[real_y-1][real_x].children.sort_by(|a, b| a.0.cmp(&b.0));
            }
            if real_y < array_size-1 {
                graph[real_y+1][real_x].children.push((this_risk, real_x, real_y));
            }
        }
    }
    // for yy in 0..array_size {//[0,1,array_size/5-1,array_size/5,array_size/5+1, array_size-1] {
    //     for xx in 0..array_size {
    //         print!("{}", graph[yy][xx].risk);
    //     }
    //     print!("\n");
    // }
    // print!("\n");

    let start_node = &graph[0][0];
    let start_cost = 0;
    //let mut shortest_path = Vec::new();
    //traverse_graph_min_risk(&graph, &mut min_risks, start_node, start_cost);
    //println!("{}", min_risks[array_size-1][array_size-1]);
    let real_cost = shortest_path(&graph, (0, 0), (array_size-1, array_size-1), array_size);
    println!("Real cost: {}", real_cost.unwrap());
}

fn main() {
    solve_part_1();
    solve_part_2();
}
