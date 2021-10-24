use rand::Rng;

const START_WEIGHTS: u32 = 3;
const VERTICES: usize = 128;

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
    edge_count: u32,
}

impl Unweighted {
    pub fn init(n: usize) -> Self {
        Self {
            adj_list: vec![vec![]; n],
            edge_count: 0,
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

    pub fn add_vertex(&mut self) -> usize {
        self.adj_list.push(vec![]);
        self.adj_list.len() - 1
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.adj_list.len());
        assert!(v < self.adj_list.len());
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

    //ring graph
    pub fn ring_graph(&mut self, mut m: usize) {
        if m > VERTICES {
            m %= VERTICES;
        }
        // self.edge_count =2 * m * VERTICES;
        // self.edge_count /= 2;
        for i in 0..=(VERTICES - 1) {
            for j in 1..=m {
                for _ in 0..=START_WEIGHTS - 1 {
                    self.adj_list[i].push((i + j) % VERTICES);
                    self.edge_count += 1;
                    self.adj_list[i].push((i + VERTICES - j) % VERTICES);
                    self.edge_count += 1;
                }
            }
        }
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
