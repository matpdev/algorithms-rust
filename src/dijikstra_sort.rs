use std::cmp::Reverse;
use std::collections::{ BTreeMap, BinaryHeap };
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[new] {
            Some((p, d)) if p == *prev && d == dist_new => {}
            _ => {
                continue;
            }
        }

        for (next, weight) in &graph[new] {
            match ans.get(next) {
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                Some(None) => {}
                _ => {
                    ans.insert(*next, Some((*new, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, next, new)));
                }
            }
        }
    }

    ans
}

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new);
}

fn tree_1() {
    let mut graph = BTreeMap::new();
    let mut dists = BTreeMap::new();
    dists.insert(1, None);
    for i in 1..100 {
        add_edge(&mut graph, i, i * 2, i * 2);
        add_edge(&mut graph, i, i * 2 + 1, i * 2 + 1);

        match dists[&i] {
            Some((_, d)) => {
                dists.insert(i * 2, Some((i, d + i * 2)));
                dists.insert(i * 2 + 1, Some((i, d + i * 2 + 1)));
            }
            None => {
                dists.insert(i * 2, Some((i, i * 2)));
                dists.insert(i * 2 + 1, Some((i, i * 2 + 1)));
            }
        }
    }

    println!("{:?}", dijkstra(&graph, &1));

    //  assert_eq!(dijkstra(&graph, &1), dists);
}

pub fn print_dijkstra() {
    tree_1();
}