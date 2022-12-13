use std::collections::HashMap;

use petgraph::{adj::NodeIndex, algo::dijkstra, Graph};

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let mut graph = Graph::new();
    let mut start_node = NodeIndex::from(0);
    let mut end_node = NodeIndex::from(0);
    let mut start_nodes_part2 = Vec::new();
    let mut node_indeces = HashMap::<(_, _), NodeIndex<_>>::new();

    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            let point = line.chars().nth(x).unwrap();
            let node: NodeIndex<_> = graph.add_node(if point == 'S' {
                1
            } else if point == 'E' {
                26
            } else {
                (point as usize) - 96
            });
            node_indeces.insert((x, y), node);
            if point == 'S' {
                start_node = node;
            }
            if point == 'E' {
                end_node = node;
            }
        }
    }
    for ((x, y), node) in node_indeces.clone() {
        let current_node = graph[node];
        if current_node == 1 {
            start_nodes_part2.push(node);
        }
        if x > 0 && current_node + 1 >= graph[node_indeces[&(x - 1, y)]] {
            graph.update_edge(node, node_indeces[&(x - 1, y)], 1);
        }
        if y > 0 && current_node + 1 >= graph[node_indeces[&(x, y - 1)]] {
            graph.update_edge(node, node_indeces[&(x, y - 1)], 1);
        }
        if x < lines[0].len() - 1 && current_node + 1 >= graph[node_indeces[&(x + 1, y)]] {
            graph.update_edge(node, node_indeces[&(x + 1, y)], 1);
        }
        if y < lines.len() - 1 && current_node + 1 >= graph[node_indeces[&(x, y + 1)]] {
            graph.update_edge(node, node_indeces[&(x, y + 1)], 1);
        }
    }
    let shortest_path = dijkstra(&graph, start_node, Some(end_node), |_| 1);
    println!("Part1: {:#?}", shortest_path[&end_node]);

    let shortest_path = start_nodes_part2
        .into_iter()
        .map(|start_node| dijkstra(&graph, start_node, Some(end_node), |_| 1))
        .map(|paths| paths.get(&end_node).unwrap_or(&i32::MAX).clone())
        .min()
        .unwrap();
    println!("Part2: {:#?}", shortest_path);
}
