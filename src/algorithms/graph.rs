#[cfg(feature = "algorithms")]
use petgraph::{graph::Graph, graph::NodeIndex, Directed, Undirected};

#[cfg(feature = "algorithms")]
pub fn create_graph() -> Graph<String, f64, Directed> {
    Graph::new()
}

#[cfg(feature = "algorithms")]
pub fn add_node(graph: &mut Graph<String, f64, Directed>, label: &str) -> NodeIndex {
    graph.add_node(label.to_string())
}

#[cfg(feature = "algorithms")]
pub fn add_edge(
    graph: &mut Graph<String, f64, Directed>,
    from: NodeIndex,
    to: NodeIndex,
    weight: f64,
) {
    graph.add_edge(from, to, weight);
}

#[cfg(feature = "algorithms")]
pub fn dijkstra_shortest_path(
    graph: &Graph<String, f64, Directed>,
    start: NodeIndex,
) -> Vec<Option<f64>> {
    use petgraph::algo::dijkstra;
    dijkstra(graph, start, None, |e| *e.weight())
}

#[cfg(not(feature = "algorithms"))]
pub fn create_graph() {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn add_node() {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn add_edge() {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn dijkstra_shortest_path() {
    panic!("algorithms feature not enabled");
}
