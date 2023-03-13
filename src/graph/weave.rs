use itertools::any;
use itertools::Itertools;

use ndarray;

use rayon::prelude::*;

use std::cell::RefCell;

use super::structs;
use super::types::{
    Adjacency, Bobbins, Count, EdgeAdjacency, Idx, Loom, Node, Point, Solution, Spool,
    Subtours, Thread, Tour, TourSlice, V2d, Varr, VIMap, Vert, Verts, VertsC3, WarpedLoom,
    Warps, Woven, Yarn, ZOrder
};

pub fn weave(
    adj: &Adjacency,
    vi_map: &VIMap,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    z_adj: &Adjacency,
    z_order: &ZOrder,
) -> Solution {
    let mut warp_wefts: Loom = warp_loom(vi_map, verts, z_adj, z_order);
    join_loops(warp_wefts.split_first_mut().unwrap(), adj, verts, edge_adj)
}

pub fn warp_loom(vi_map: &VIMap, verts: &Verts, z_adj: &Adjacency, z_order: &ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(&z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, verts, &vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn spin_and_color_yarn(z_adj: &Adjacency, verts: &Verts) -> Spool {
    let natural: Yarn = spin(&z_adj, verts);
    let colored: Yarn = color_yarn(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn spin(z_adj: &Adjacency, verts: &Verts) -> Yarn {
    let var: Varr = from_verts_to_vertsc2d(verts);
    let path: &mut Tour = &mut vec![*z_adj.keys().max().unwrap() as Node];
    let order: Count = z_adj.len();
    (1..order).for_each(|idx| path.push(get_next(&path, z_adj, &var, idx, order)));
    from_nodes_to_yarn(path, verts)
}

pub fn from_verts_to_vertsc2d(verts: &Verts) -> Varr {
    verts.par_iter().map(|(_x, _y, _)| [*_x, *_y]).collect()
}

pub fn get_next(path: TourSlice, adj: &Adjacency, verts: &Varr, idx: usize, order: usize) -> Node {
    if idx < order - 5 {
        adj[path.last().unwrap()]
            .iter()
            .filter(|n| !path.contains(*n))
            .copied()
            .max_by_key(|&n| absumv(verts[n as usize]))
            .unwrap()
    } else {
        let curr: &Node = &path[path.len() - 1];
        let curr_vert: &V2d = &verts[*curr as usize];
        adj[curr]
            .iter()
            .filter(|n| !path.contains(*n))
            .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
            .filter(|(_, next_axis)| {
                *next_axis != get_axis(&verts[path[path.len() - 2] as usize], curr_vert)
            })
            .max_by_key(|&(n, _)| absumv(verts[n as usize]))
            .unwrap()
            .0
    }
}

pub fn get_axis(m_vert: &V2d, n_vert: &V2d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("Something's wrong, the same verts are being compared.")
}

pub fn absumv(vert: V2d) -> Point {
    vert.iter()
        .map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31))
        .sum()
}

pub fn from_nodes_to_yarn(path: &mut Tour, verts: &Verts) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

pub fn color_yarn(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

pub fn wind(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.iter_mut()
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

pub fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
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

pub fn join_loops(
    (warp, wefts): (&mut Thread, &mut [Thread]),
    adj: &Adjacency,
    verts: &VertsC3,
    edge_adj: &EdgeAdjacency,
) -> Solution {
    let warp: &mut structs::Cycle = structs::Cycle::new(warp, &adj, &edge_adj, verts);
    let loom: WarpedLoom = wefts
        .iter()
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