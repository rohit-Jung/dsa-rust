#![allow(dead_code, unused_imports)]

use rust::{
    graphs::matrix_rep::Graph,
    knapsack::{fractional::fractional_knapsack, zeroandone},
    sortings::{bubble_sort, insertion_sort, merge_sort::merge_sort, quick_sort, selection_sort},
};

// use rust::graphs::list_rep::ListGraph;
fn main() {
    println!("Hello, DSA!");
    // run_sortings();
    // run_knapsack();
    run_graphs();
}

fn run_graphs() {
    // let mut graph = Graph::new(6);
    // graph.add_edge(0, 1);
    // graph.add_edge(0, 2);
    // graph.add_edge(2, 3);
    // graph.add_edge(2, 4);
    // graph.add_edge(3, 5);
    // graph.add_edge(4, 5);
    // // graph.print_graph();
    // graph.dfs(0);

    let mut graph = Graph::new(8);
    graph.add_edge(0, 1);
    graph.add_edge(0, 3);
    graph.add_edge(1, 3);
    graph.add_edge(3, 5);
    graph.add_edge(3, 4);
    graph.add_edge(4, 5);
    graph.add_edge(4, 6);
    graph.add_edge(6, 7);
    graph.add_edge(6, 2);
    // graph.print_graph();
    graph.bfs(0);
    println!();
    graph.dfs(0);

    let mut seen: Vec<bool> = vec![false; graph.size];
    let mut prev: Vec<usize> = vec![usize::MAX; graph.size];

    graph.dfs_rec(0, &mut seen, &mut prev);
    println!("{:?}", prev);

    // search through
    let mut curr = 5;
    let mut out: Vec<usize> = Vec::new();
    while prev[curr] != usize::MAX {
        out.push(curr);
        curr = prev[curr]
    }

    // 0 is the source
    out.push(0);
    out.reverse();
    println!("out {:?}", out);

    // let mut graph = ListGraph::new(5);
    // graph.add_edge(0, 1, true);
    // graph.add_edge(0, 2, true);
    // graph.add_edge(1, 3, true);
    // graph.add_edge(2, 3, true);
    // graph.add_edge(3, 4, true);
    // graph.add_edge(2, 3, true);
    // graph.print_graph();
}

fn run_sortings() {
    // sortings
    let mut unsorted = [3, 1, 4, 2, 5];
    println!("{:?}", unsorted);
    // bubble_sort::bubble_sort(&mut unsorted);
    // selection_sort::selection_sort(&mut unsorted);
    // insertion_sort::insertion_sort(&mut unsorted);
    // quick_sort::quick_sort(&mut unsorted);
    merge_sort(&mut unsorted);
    println!("{:?}", unsorted);
}

fn run_knapsack() {
    // let values = vec![24, 21, 12, 10];
    // let weights = vec![7, 3, 4, 5];
    // let capacity = 20;

    let values = vec![2, 3, 4, 5];
    let weights = vec![3, 4, 5, 6];
    let capacity = 5;

    // fractional_knapsack(values, weights, capacity);
    zeroandone::zeroone_knapsack(values, weights, capacity);
}
