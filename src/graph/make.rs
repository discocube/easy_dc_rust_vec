use itertools::Itertools;
use ndarray::arr2;

use super::check::is_valid_edge;
use super::types::{
    Adjacency, EdgeAdjacency, Edges, Idx, Node, Nodes, Point, VIMap, Verts, VertsC3, Weights,
};
use super::utils::{absumv, edist, shift_xyz};

pub fn vertices(max_xyz: Point) -> Verts {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-max_xyz..=max_xyz)
                .step_by(2)
                .flat_map(move |y| {
                    (-max_xyz..=max_xyz)
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        .filter(|&v| absumv(v) < (max_xyz + 4))
                        .collect::<Verts>()
                })
                .collect::<Verts>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist(*v), v.0, v.1, v.2))
        .collect()
}

pub fn vi_map(verts: &Verts) -> VIMap {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as Node))
        .collect()
}

pub fn adjacency_map(verts: &VertsC3, max_xyz: Point, vi: &VIMap) -> Adjacency {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| {
            (
                idx as Node,
                shift_xyz(arr2(&[[vert.0, vert.1, vert.2]]))
                    .outer_iter()
                    .filter(|new_vert| {
                        absumv((new_vert[0], new_vert[1], new_vert[2])) <= max_xyz + 2
                    })
                    .map(|new_vert| vi[&(new_vert[0], new_vert[1], new_vert[2])])
                    .filter(|&m| m != (idx as Node))
                    .collect::<Nodes>(),
            )
        })
        .collect()
}

pub fn edges_from_adjacency(adj: &Adjacency) -> Edges {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

pub fn edges_adjacency_map(adj: &Adjacency, edges: &Edges, verts: &Verts) -> EdgeAdjacency {
    edges
        .iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as Idx], verts[*b as Idx]))
        .map(|&(m, n)| ((m, n), adjacent_edges(adj, m, n, verts)))
        .collect()
}

pub fn adjacent_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj[&m_node]
        .iter()
        .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| {
            adj[m].contains(n) && is_valid_edge(verts[*m as Idx], verts[*n as Idx])
        })
        .map(|(m, n)| if m < n { (m, n) } else { (n, m) })
        .collect()
}

pub fn weights_map(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.iter()
        .map(|(&n, _)| (n, absumv(verts[n as usize])))
        .collect()
}
