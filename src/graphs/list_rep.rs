use std::collections::HashMap;

pub struct ListGraph {
    adj_list: HashMap<usize, Vec<usize>>,
}

impl ListGraph {
    pub fn new(vertices: usize) -> Self {
        ListGraph {
            adj_list: HashMap::with_capacity(vertices),
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize, undirected: bool) {
        self.adj_list.entry(src).or_default().push(dest);
        if undirected {
            self.adj_list.entry(dest).or_default().push(src);
        }
    }

    pub fn print_graph(&self) {
        for (key, value) in &self.adj_list {
            println!("{}: {:?}", key, value);
        }
    }
}
