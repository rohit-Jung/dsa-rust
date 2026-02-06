#![allow(dead_code, unused_imports)]

use rust::{
    graphs::matrix_rep::Graph,
    knapsack::fractional::fractional_knapsack,
    sortings::{bubble_sort, insertion_sort, quick_sort, selection_sort},
};

// use rust::graphs::list_rep::ListGraph;
fn main() {
    println!("Hello, DSA!");
    run_knapsack();
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
    let mut unsorted = [5, 4, 3, 2, 1];
    println!("{:?}", unsorted);
    // bubble_sort::bubble_sort(&mut unsorted);
    // selection_sort::selection_sort(&mut unsorted);
    // insertion_sort::insertion_sort(&mut unsorted);
    quick_sort::quick_sort(&mut unsorted);
    println!("{:?}", unsorted);
}

fn run_knapsack() {
    let values = vec![24, 21, 12, 10];
    let weights = vec![7, 3, 4, 5];
    let capacity = 20;

    fractional_knapsack(values, weights, capacity);
}
