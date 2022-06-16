use std::iter::Map;

pub struct Graph {
    no_vertices: i64,
    no_edges: i64,

    adj: Map<Vertex<i64>, Vec<Edge>>,
}

pub fn initGraph<T: PartialEq>() -> Graph {
    Graph {
        no_vertices: 0,
        no_edges: 0,
        adj: Map::new(iter: I, f: F),
    }
}

impl<T: PartialEq> Graph<T> {
    pub fn add_edge(&mut self, from: i64, to: i64) {
        self.adj.push(Edge { from, to });
    }
}

#[derive(Clone)]
struct Vertex<T: PartialEq> {
    value: T,
    edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
struct Edge {
    from: i64,
    to: i64,
}
