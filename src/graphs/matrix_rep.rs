use std::{collections::VecDeque, vec};

pub struct Graph {
    size: usize,
    mat: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            mat: vec![vec![false; size]; size],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.size && v < self.size {
            self.mat[u][v] = true;
            self.mat[v][u] = true;
        } else {
            println!("Invalid Edge");
        }
    }

    pub fn print_graph(&self) {
        for row in &self.mat {
            // checking
            // println!("{:?}", row);
            for &elem in row {
                let num: u8 = elem.into();
                print!("{num} ");
            }
            println!()
        }
    }

    // TODO: Implement these
    pub fn dfs(&self, starting_vertex: usize) {}

    pub fn bfs(&self, starting_vertex: usize) {}
}
