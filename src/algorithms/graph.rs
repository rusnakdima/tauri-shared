#[cfg(feature = "algorithms")]
use petgraph::{graph::Graph, graph::NodeIndex, Directed};

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
) -> std::collections::HashMap<NodeIndex, f64> {
    use petgraph::algo::dijkstra;
    dijkstra(graph, start, None, |e| *e.weight())
}

#[cfg(not(feature = "algorithms"))]
pub fn create_graph() {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn add_node(_label: &str) {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn add_edge(_from: usize, _to: usize, _weight: f64) {
    panic!("algorithms feature not enabled");
}

#[cfg(not(feature = "algorithms"))]
pub fn dijkstra_shortest_path() -> std::collections::HashMap<usize, f64> {
    panic!("algorithms feature not enabled");
}
