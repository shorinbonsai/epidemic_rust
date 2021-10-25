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
#[derive(Debug, Clone)]
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
        for i in 0..VERTICES {
            for j in 1..=m {
                for _ in 0..START_WEIGHTS {
                    self.adj_list[i].push((i + j) % VERTICES);
                    self.edge_count += 1;
                    self.adj_list[i].push((i + VERTICES - j) % VERTICES);
                    self.edge_count += 1;
                }
            }
        }
    }

    pub fn SIR(&self, p0: usize, alpha: f64) -> (u32, u32, u32) {
        if p0 >= VERTICES {
            return (0, 0, 0);
        }
        let mut max = 0;
        let mut len = 0;
        let mut ttl = 0;
        let mut nin: [u32; VERTICES] = [0; VERTICES]; //infected neibours counters
        let mut clr = Unweighted::set_colour(0); // set population to susceptible
        clr[p0] = 1; //infect patient zero
        let mut numb_inf = 1; // initialize to one person currently infected
        while numb_inf > 0 {
            for i in 0..VERTICES {
                nin[i] = 0; //zero the number of infected neighbors buffer
            }
            for i in 0..VERTICES {
                if clr[i] == 1 {
                    //found infected individual
                    for j in 0..self.adj_list[i].len() {
                        nin[self.adj_list[i][j]] += 1; //record exposure
                    }
                }
            }
            //check for transmission
            for i in 0..VERTICES {
                if clr[i] == 0 && nin[i] > 0 {
                    clr[i] = 3 * Unweighted::infected(nin[i], alpha);
                }
            }
            if numb_inf > max {
                max = numb_inf;
            }
            ttl += numb_inf;
            numb_inf = 0;
            for i in 0..VERTICES {
                match clr[i] {
                    0 => (),         //susceptible, do nothing
                    1 => clr[i] = 2, //infected, move to removed
                    2 => (),         //removed, do nothing
                    3 => {
                        //newly infected
                        clr[i] = 1;
                        numb_inf += 1;
                    }
                    _ => (),
                }
            }
            len += 1; //record time step
        }
        // unimplemented!()
        (max, len, ttl)
    }

    pub fn set_colour(value: u32) -> [u32; VERTICES] {
        let clr: [u32; VERTICES] = [value; VERTICES];
        clr
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
