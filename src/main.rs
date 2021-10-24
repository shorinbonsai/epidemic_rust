use crate::graph::Unweighted;
mod graph;

fn main() {
    println!("Hello, world!");
    let mut graph = Unweighted::new_undirected(
        13,
        &[
            [0, 7],
            [0, 9],
            [0, 11],
            [7, 11],
            [7, 6],
            [7, 3],
            [6, 5],
            [3, 4],
            [2, 3],
            [2, 12],
            [12, 8],
            [8, 1],
            [1, 10],
            [10, 9],
            [9, 8],
        ],
    );

    let mut graph2 = Unweighted::new_undirected(128, &[]);
    println!("{:?}", graph2);
    graph2.ring_graph(2);
    println!("{:?}", graph2);

    let x = Unweighted::infected(10, 0.15);
    println!("{}", x);
    // println!("{:?}", graph);
    // graph.remove_edge(0, 11);
    // println!("{:?}", graph);
    // graph.remove_edge(11, 0);
    // println!("{:?}", graph);
    // graph.remove_edges(0, 7);
    // println!("{:?}", graph);
    // graph.add_edge(0, 11);
    // graph.add_undirected_edge(7, 0);
    // graph.add_undirected_edge(7, 0);
    // println!("{:?}", graph);
    // println!("{:?}", graph.is_empty());
    // graph.clear_graph();
    // println!("{:?}", graph);
    // graph.kill_graph();
    // println!("{:?}", graph.is_empty());
}
