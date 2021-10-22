use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}

impl Edge {
    /// Consruct a new (directed) weighted `Edge`
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

/// An unweighted graph represented by an unweighted adjacency list.
#[derive(Debug)]
pub struct Unweighted {
    adj_list: Vec<Vec<usize>>,
}

impl Unweighted {
    pub fn init(n: usize) -> Self {
        Self {
            adj_list: vec![vec![]; n],
        }
    }

    pub fn new_undirected(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::init(size);
        for &[a, b] in edges.iter() {
            graph.add_undirected_edge(a, b);
        }
        graph
    }

    pub fn clear_graph(&mut self) {
        for x in self.adj_list.iter_mut() {
            x.clear();
        }
    }

    pub fn kill_graph(&mut self) {
        self.adj_list.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.adj_list.is_empty()
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v)
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        if let Some(pos) = self.adj_list[u].iter().position(|x| *x == v) {
            self.adj_list[u].remove(pos);
        }
    }

    pub fn remove_edges(&mut self, u: usize, v: usize) {
        self.remove_edge(u, v);
        self.remove_edge(v, u);
    }

    pub fn infected(n: u32, alpha: f64) -> u32 {
        let mut rng = rand::thread_rng();
        let beta: f64 = 1.0 - (f64::from(n) * (1.0 - alpha).ln()).exp();
        let x = rng.gen::<f64>();
        if x < beta {
            return 1;
        } else {
            return 0;
        }
    }
}
