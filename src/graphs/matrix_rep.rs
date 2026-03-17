use std::{collections::VecDeque, vec};

pub struct Graph {
    pub size: usize,
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

    // Why BFS - fairness - queue
    pub fn dfs(&self, starting_vertex: usize) {
        // dfs is done using stack
        let mut stack: Vec<usize> = Vec::new();
        let mut visited = vec![false; self.size];

        stack.push(starting_vertex);

        while let Some(elem) = stack.pop() {
            if visited[elem] {
                continue;
            }

            // mark as visited ?
            println!("{}", elem);
            visited[elem] = true;

            for (i, &connected) in self.mat[elem].iter().enumerate() {
                if connected && !visited[i] {
                    stack.push(i);
                }
            }
        }
    }

    pub fn bfs(&self, starting_vertex: usize) {
        // bfs is done using queue
        let mut visited = vec![false; self.size];
        let mut queue = VecDeque::new();
        queue.push_back(starting_vertex);

        while let Some(elem) = queue.pop_front() {
            if visited[elem] {
                continue;
            }

            println!("{}", elem);
            visited[elem] = true;

            for (i, &conn) in self.mat[elem].iter().enumerate() {
                if conn && !visited[i] {
                    queue.push_back(i);
                }
            }
        }
    }

    pub fn dfs_rec(&self, node: usize, seen: &mut Vec<bool>, prev: &mut Vec<usize>) {
        if seen[node] {
            return;
        };

        seen[node] = true;

        for (i, &connected) in self.mat[node].iter().enumerate() {
            if connected && !seen[i] {
                prev[i] = node;
                self.dfs_rec(i, seen, prev);
            }
        }
    }
}
