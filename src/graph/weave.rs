extern crate itertools;

use itertools::any;
use itertools::Itertools;

use ndarray;
use ndarray::Array2;

use rayon::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

use super::cycle_var;
use super::structs;
use super::types::Spool16;
use super::types::V3d16;
use super::types::VIMapVar16;
use super::types::Yarn16;
use super::types::ZOrderVar;
use super::types::{
    Adjacency, Bobbins, Count, EdgeAdjacency, Idx, Loom, Node, Point, Solution, Spool,
    Subtours, Thread, Tour, TourSlice, V3d, VIMap, Varr, Vert, Verts, VertsC3, Warps,
    Woven, Yarn,
};
use super::utils::absumv_i;

// pub fn weave(
//     adj: &Adjacency,
//     vi_map: &VIMap,
//     edge_adj: &EdgeAdjacency,
//     verts: &Verts,
//     z_adj: &Adjacency,
//     z_order: &ZOrder,
// ) -> Solution {
//     let mut warp_wefts: Loom = warp_loom(vi_map, verts, z_adj, z_order);
//     join_loops(warp_wefts.split_first_mut().unwrap(), adj, verts, edge_adj)
// }

// pub fn warp_loom(vi_map: &VIMap, verts: &Verts, z_adj: &Adjacency, z_order: &ZOrder) -> Loom {
//     let spool: Spool = yarn(&z_adj, verts);
//     let mut bobbins: Bobbins = Vec::new();
//     let mut loom: Loom = Loom::new();
//     for (zlevel, order) in z_order {
//         let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
//         let woven: Woven = join_threads(&mut loom, &warps);
//         affix_loose_threads(&mut loom, warps, woven);
//         if *zlevel != -1 {
//             bobbins = wind(&mut loom, verts, &vi_map);
//         }
//     }
//     reflect_loom(&mut loom, verts, vi_map);
//     loom.sort_by_key(|w| w.len());
//     loom
// }

pub fn weave_var(
    adj: &Adjacency,
    vi_map: &VIMapVar16,
    edge_adj: &EdgeAdjacency,
    verts: &Varr,
    z_adj: &Adjacency,
    z_order: &ZOrderVar,
) -> Solution {
    let mut warp_wefts: Loom = warp_loom_var(vi_map, verts, z_adj, z_order);
    join_loops_var(warp_wefts.split_first_mut().unwrap(), adj, verts, edge_adj)
}

pub fn warp_loom_var(vi_map: &VIMapVar16, verts: &Varr, z_adj: &Adjacency, z_order: &ZOrderVar) -> Loom {
    let spool: Spool16 = yarn_var(&z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        let warps: Warps = get_warps_var(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind_var(&mut loom, verts, &vi_map);
        }
    }
    reflect_loom_var(&mut loom, verts, vi_map);
    loom.sort_by_key(|w| w.len());
    loom
}

// pub fn yarn(z_adj: &Adjacency, verts: &Verts) -> Spool {
//     let natural: Yarn = spin(&z_adj, verts);
//     let colored: Yarn = color(&natural);
//     Spool::from([(3, natural), (1, colored)])
// }

pub fn yarn_var(z_adj: &Adjacency, verts: &Varr) -> Spool16 {
    let natural: Yarn16 = spin_var(&z_adj, verts);
    let colored: Yarn16 = color16(&natural);
    Spool16::from([(3, natural), (1, colored)])
}

// pub fn spin(adj: &Adjacency, verts: &Verts) -> Yarn16 {
//     let var: Varr = convert::from_verts_to_vertsc(verts);
//     let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
//     let order: Count = adj.len();
//     (1..order).for_each(|idx| path.push(get_next(&path, adj, &var, idx, order)));
//     from_nodes_to_yarn(path, verts)
// }

pub fn spin_var(adj: &Adjacency, verts: &Varr) -> Yarn16 {
    let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
    let order: Count = adj.len();
    (1..order).for_each(|idx| path.push(get_next_var(&path, adj, verts, idx, order)));
    from_nodes_to_yarn_var(path, verts)
}

// pub fn get_next(path: TourSlice, adj: &Adjacency, verts: &Varr, idx: usize, order: usize) -> Node {
//     if idx < order - 5 {
//         adj[path.last().unwrap()]
//             .iter()
//             .filter(|n| !path.contains(*n))
//             .copied()
//             .max_by_key(|&n| absumv_v3d(verts[n as usize]))
//             .unwrap()
//     } else {
//         let curr: &Node = &path[path.len() - 1];
//         let curr_vert: &V3d = &verts[*curr as usize];
//         adj[curr]
//             .iter()
//             .filter(|n| !path.contains(*n))
//             .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
//             .filter(|(_, next_axis)| {
//                 *next_axis != get_axis(&verts[path[path.len() - 2] as usize], curr_vert)
//             })
//             .max_by_key(|&(n, _)| absumv_v3d(verts[n as usize]))
//             .unwrap()
//             .0
//     }
// }

pub fn get_next_var(path: TourSlice, adj: &Adjacency, verts: &Varr, idx: usize, order: usize) -> Node {
    if idx < order - 5 {
        adj[path.last().unwrap()]
            .iter()
            .filter(|n| !path.contains(*n))
            .copied()
            .max_by_key(|&n| absumv_i(verts[n as usize]))
            .unwrap()
    } else {
        let curr: &Node = &path[path.len() - 1];
        let curr_vert: &V3d16 = &verts[*curr as usize];
        adj[curr]
            .iter()
            .filter(|n| !path.contains(*n))
            .map(|&n| (n, get_axis16(curr_vert, &verts[n as usize])))
            .filter(|(_, next_axis)| {
                *next_axis != get_axis16(&verts[path[path.len() - 2] as usize], curr_vert)
            })
            .max_by_key(|&(n, _)| absumv_i(verts[n as usize]))
            .unwrap()
            .0
    }
}

pub fn get_axis(m_vert: &V3d, n_vert: &V3d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("Something's wrong, the same verts are being compared.")
}

pub fn get_axis16(m_vert: &V3d16, n_vert: &V3d16) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("Something's wrong, the same verts are being compared.")
}

pub fn from_nodes_to_yarn(path: &mut Vec<u32>, verts: &Verts) -> Yarn {
    Yarn::from(
        path.par_iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

pub fn from_nodes_to_yarn_var(path: &mut Vec<u32>, verts: &Varr) -> Array2<i16> {
    Array2::from(
        path.par_iter()
            .map(|&n| [verts[n as usize][0], verts[n as usize][1]])
            .collect::<Vec<[i16; 2]>>(),
    )
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

pub fn color16(a: &Yarn16) -> Yarn16 {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

pub fn wind(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.par_iter_mut()
        .map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vi_map,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect()
}

pub fn wind_var(loom: &mut Loom, verts: &Varr, vi_map: &VIMapVar16) -> Bobbins {
    loom.par_iter_mut()
        .map(|thread| {
            let (left, right) = get_upper_nodes_var(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vi_map,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect()
}

pub fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
}

pub fn get_upper_nodes_var([x, y, z]: [i16; 3], [x1, y1, z1]: [i16; 3], vi_map: &VIMapVar16) -> (u32, u32) {
    (vi_map[&[x, y, z + 2]], vi_map[&[x1, y1, z1 + 2]])
}

pub fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let node_yarn: Tour = get_node_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, &bobbins)
    }
}

pub fn get_warps_var(
    zlevel: i16,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool16,
    vi_map: &VIMapVar16,
) -> Warps {
    let node_yarn: Tour = get_node_yarn_var(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, &bobbins)
    }
}

pub fn get_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vi_map: &VIMap) -> Tour {
    yarn.slice_axis_inplace(
        ndarray::Axis(0),
        ndarray::Slice::new(
            (yarn.len_of(ndarray::Axis(0)) - order).try_into().unwrap(),
            None,
            1,
        ),
    );
    yarn.outer_iter()
        .map(|row| vi_map[&(row[0], row[1], zlevel)])
        .collect()
}

pub fn get_node_yarn_var(mut yarn: Yarn16, zlevel: i16, order: Count, vi_map: &VIMapVar16) -> Tour {
    yarn.slice_axis_inplace(
        ndarray::Axis(0),
        ndarray::Slice::new(
            (yarn.len_of(ndarray::Axis(0)) - order).try_into().unwrap(),
            None,
            1,
        ),
    );
    yarn.outer_iter()
        .map(|row| vi_map[&[row[0], row[1], zlevel as i16]])
        .collect()
}

pub fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1 as i32;
    for (e, idx) in tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if subset.contains(&node) {
                Some(i)
            } else {
                None
            }
        })
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            for subtour in vec![tour[prev as usize + 1..idx].to_vec(), tour[idx..].to_vec()] {
                if !subtour.is_empty() {
                    subtours.push(if subset.contains(&subtour[0]) {
                        subtour
                    } else {
                        subtour.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            let subtour = tour[prev as usize + 1..=idx].to_vec();
            if !subtour.is_empty() {
                subtours.push(if subset.contains(&subtour[0]) {
                    subtour
                } else {
                    subtour.iter().rev().cloned().collect()
                });
            }
            prev = idx as i32;
        }
    }
    subtours
}

pub fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
    let mut woven: Woven = Woven::new();
    for thread in loom {
        for (idx, warp) in warps.iter().enumerate() {
            if !woven.contains(&idx) {
                match (thread.front(), thread.back()) {
                    (Some(front), _) if *front == warp[0] => {
                        *thread = warp[1..]
                            .iter()
                            .rev()
                            .chain(thread.iter())
                            .cloned()
                            .collect()
                    }
                    (_, Some(back)) if *back == warp[0] => thread.extend(warp.iter().skip(1)),
                    _ => continue,
                }
                woven.extend([idx])
            }
        }
    }
    woven
}

pub fn affix_loose_threads(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![Thread::from(seq.iter().cloned().collect::<Thread>())])
    }
}

pub fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vi_map[&(x, y, -z)])
                .collect::<Tour>(),
        )
    });
}

pub fn reflect_loom_var(loom: &mut Loom, verts: &Varr, vi_map: &VIMapVar16) {
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|[x, y, z]| vi_map[&[x, y, -z]])
                .collect::<Tour>(),
        )
    });
}

pub fn join_loops(
    (warp, wefts): (&mut VecDeque<u32>, &mut [VecDeque<u32>]),
    adj: &Adjacency,
    verts: &VertsC3,
    edge_adj: &EdgeAdjacency,
) -> Solution {
    let warp: &mut structs::Cycle = structs::Cycle::new(warp, &adj, &edge_adj, verts);
    let loom: HashMap<_, _> = wefts
        .par_iter()
        .enumerate()
        .map(|(idx, seq)| {
            (
                idx,
                RefCell::new(structs::Cycle::new(&seq, &adj, &edge_adj, verts)),
            )
        })
        .collect();
    while any(loom.values(), |cycle| !cycle.borrow_mut().is_empty) {
        for _other in loom
            .values()
            .filter(|cycle_refcell| !cycle_refcell.borrow_mut().is_empty)
        {
            let mut other = _other.borrow_mut();
            if let Some(warp_e) = warp.edges().intersection(&other.eadjs()).next() {
                if let Some(weft_e) = edge_adj
                    .get(&warp_e)
                    .unwrap()
                    .intersection(&other.edges())
                    .next()
                {
                    warp.join(*warp_e, *weft_e, &mut other);
                }
            }
        }
    }
    warp.retrieve()
}


pub fn join_loops_var(
    (warp, wefts): (&mut VecDeque<u32>, &mut [VecDeque<u32>]),
    adj: &Adjacency,
    verts: &Varr,
    edge_adj: &EdgeAdjacency,
) -> Solution {
    let warp: &mut cycle_var::Cycle = cycle_var::Cycle::new(warp, &adj, &edge_adj, verts);
    let loom: HashMap<_, _> = wefts
        .par_iter()
        .enumerate()
        .map(|(idx, seq)| {
            (
                idx,
                RefCell::new(cycle_var::Cycle::new(&seq, &adj, &edge_adj, verts)),
            )
        })
        .collect();
    while any(loom.values(), |cycle| !cycle.borrow_mut().is_empty) {
        for _other in loom
            .values()
            .filter(|cycle_refcell| !cycle_refcell.borrow_mut().is_empty)
        {
            let mut other = _other.borrow_mut();
            if let Some(warp_e) = warp.edges().intersection(&other.eadjs()).next() {
                if let Some(weft_e) = edge_adj
                    .get(&warp_e)
                    .unwrap()
                    .intersection(&other.edges())
                    .next()
                {
                    warp.join(*warp_e, *weft_e, &mut other);
                }
            }
        }
    }
    warp.retrieve()
}