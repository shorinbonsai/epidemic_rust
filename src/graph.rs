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
    graph: Vec<Vec<usize>>,
}

impl Unweighted {
    pub fn init(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v)
    }
}
