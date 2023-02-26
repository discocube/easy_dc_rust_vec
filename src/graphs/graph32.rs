use std::collections::{HashMap, HashSet};
use common_macros::hash_map;

use crate::types::types::{Adj, Neighbors};

pub const GRAPH: [(u32, &[u32]); 32] = [    
    (0, &[1, 2, 4, 8, 12, 14]),
    (1, &[0, 3, 5, 9, 13, 15]),
    (2, &[0, 3, 6, 10, 16, 18]),
    (3, &[1, 2, 7, 11, 17, 19]),
    (4, &[0, 5, 6, 20, 22, 28]),
    (5, &[1, 4, 7, 21, 23, 29]),
    (6, &[2, 4, 7, 24, 26, 30]),
    (7, &[3, 5, 6, 25, 27, 31]),
    (8, &[0, 9, 10]),
    (9, &[1, 8, 11]),
    (10, &[2, 8, 11]),
    (11, &[3, 9, 10, 3]),
    (12, &[0, 20, 13]),
    (13, &[1, 12, 21]),
    (14, &[0, 16, 22]),
    (15, &[1, 17, 23]),
    (16, &[24, 2, 14]),
    (17, &[25, 3, 15]),
    (18, &[19, 2, 26]),
    (19, &[27, 18, 3]),
    (20, &[21, 4, 12]),
    (21, &[13, 20, 5]),
    (22, &[24, 4, 14]),
    (23, &[25, 5, 15]),
    (24, &[16, 6, 22]),
    (25, &[17, 23, 7]),
    (26, &[18, 27, 6]),
    (27, &[26, 19, 7]),
    (28, &[4, 29, 30]),
    (29, &[28, 5, 31]),
    (30, &[28, 6, 31]),
    (31, &[29, 30, 7]),
];


pub const GRAPH_LVL: [(u32, &[u32]); 12] = [
    (0, &[1, 2, 4, 6]),
    (1, &[0, 5, 7, 3]),
    (2, &[0, 8, 10, 3]),
    (3, &[1, 9, 2, 11]),
    (4, &[0, 5]),
    (5, &[4, 1]),
    (6, &[0, 8]),
    (7, &[1, 9]),
    (8, &[2, 6]),
    (9, &[7, 3]),
    (10, &[2, 11]),
    (11, &[10, 3]),
];

pub fn graph_to_map(graph: &[(u32, &[u32])]) -> HashMap<u32, HashSet<u32>> {
    graph.iter()
         .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect()))
         .collect()
}

pub fn graph_to_map_ref<'a>(graph: &'a [(u32, &'a [u32])]) -> HashMap<&'a u32, HashSet<&'a u32>> {
    graph.iter().map(|(node, neighbors)| {
        let neighbor_set: HashSet<&'a u32> = neighbors.iter().map(|n| n).collect();
        (node, neighbor_set)
    }).collect()
}

pub fn graph32() -> Adj {
    let graph = hash_map! {
        1 => Neighbors::Six([1, 2, 3, 4, 5, 6]),
        2 => Neighbors::Three([1, 2, 3]),
        3 => Neighbors::Six([1, 2, 3, 4, 5, 6]),
        4 => Neighbors::Three([1, 5, 6]),
    };
    graph
}