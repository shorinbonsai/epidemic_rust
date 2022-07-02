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
        self.edge_count += 1
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

    pub fn reopen_fitness(
        &self,
        p0: usize,
        nodes: u32,
        alpha: f64,
        shut_percent: f64,
        reopen: f64,
    ) -> (u32, u32, u32) {
        if p0 >= VERTICES {
            return (0, 0, 0);
        }
        let mut max: u32 = 0;
        let mut len: u32 = 0;
        let mut ttl: u32 = 0;
        let mut have_locked_down = false;
        let mut have_reopened = false;
        let mut nin: [u32; VERTICES] = [0; VERTICES]; //infected neibours counters
        let mut clr = Unweighted::set_colour(0); // set population to susceptible
        clr[p0] = 1; //infect patient zero
        let mut numb_inf: u32 = 1; // initialize to one person currently infected
        while numb_inf > 0 {
            let current_infected = numb_inf / nodes;
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
        (max, len, ttl)
    }

    /*
    def fitness_reopen(adj_lists: list[int], nodes: int, p0, remove_list: list[int] = [], edge_list: list = []):
    temp_list = copy.deepcopy(adj_lists)
    n_state = [0 for _ in range(nodes)]  # susceptible
    n_state[p0] = 1
    epi_log = [[p0]]
    num_infected = 1
    ttl_infected = 0
    time_step = 0
    have_locked_down = False
    have_reopened = False
    lockdown_step = 0
    reopen_step = 128
    length = 0
    while num_infected > 0:
        current_infected = num_infected/nodes
        if current_infected >= shutdown_percent and have_locked_down == False:
            # lockdown
            for idx, edge in enumerate(remove_list):
                if edge == 0:
                    x = edge_list[idx][0]
                    y = edge_list[idx][1]
                    temp_list = remove_edge_adj(x, y, temp_list)

            have_locked_down = True
            lockdown_step = time_step

        inf_neighbours = [0 for _ in range(nodes)]

        # if threshold met then restore initial contact graph
        current_infected = num_infected/nodes
        if current_infected < reopen_percent and have_locked_down == True and have_reopened == False:
            temp_list = copy.deepcopy(adj_lists)
            reopen_step = time_step
            have_reopened = True

        for n in range(nodes):
            if n_state[n] == 1:
                for nei in temp_list[n]:
                    inf_neighbours[nei] += 1
                    pass
                pass
            pass

        for n in range(nodes):
            if n_state[n] == 0 and inf_neighbours[n] > 0:
                if infected(inf_neighbours[n]):
                    n_state[n] = 3
                    pass
                pass
            pass

        ttl_infected += num_infected
        num_infected = 0
        new_inf = []
        for n in range(nodes):
            if n_state[n] == 1:  # infected -> removed
                n_state[n] = 2
                pass
            elif n_state[n] == 3:
                n_state[n] = 1
                num_infected += 1
                new_inf.append(n)
                pass
            pass
        epi_log.append(new_inf)
        length += 1
        time_step += 1
        pass
    return epi_log, ttl_infected, lockdown_step, reopen_step
    # return ttl_infected
     */

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
