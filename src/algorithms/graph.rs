use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
  pub id: String,
  pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
  pub from: String,
  pub to: String,
  pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
  pub nodes: Vec<GraphNode>,
  pub edges: Vec<GraphEdge>,
}

impl Default for Graph {
  fn default() -> Self {
    Self::new()
  }
}

impl Graph {
  pub fn new() -> Self {
    Graph {
      nodes: Vec::new(),
      edges: Vec::new(),
    }
  }

  pub fn add_node(&mut self, node_id: &str, data: serde_json::Value) {
    if !self.nodes.iter().any(|n| n.id == node_id) {
      self.nodes.push(GraphNode {
        id: node_id.to_string(),
        data,
      });
    }
  }

  pub fn add_edge(&mut self, from: &str, to: &str, weight: f64) {
    if !self.edges.iter().any(|e| e.from == from && e.to == to) {
      self.edges.push(GraphEdge {
        from: from.to_string(),
        to: to.to_string(),
        weight,
      });
    }
  }

  pub fn dijkstra_shortest_path(&self, start: &str, end: &str) -> Option<(Vec<String>, f64)> {
    #[derive(Clone, PartialEq)]
    struct DistNode {
      dist: f64,
      node: usize,
    }

    impl Eq for DistNode {}

    impl PartialOrd for DistNode {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist.partial_cmp(&other.dist)?.reverse())
      }
    }

    impl Ord for DistNode {
      fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
      }
    }

    let node_ids: Vec<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
    let mut node_index: HashMap<&str, usize> = HashMap::new();
    for (i, id) in node_ids.iter().enumerate() {
      node_index.insert(*id, i);
    }

    if !node_index.contains_key(start) || !node_index.contains_key(end) {
      return None;
    }

    let n = self.nodes.len();
    let start_idx = node_index[start];
    let end_idx = node_index[end];

    let mut dist: Vec<f64> = vec![f64::INFINITY; n];
    dist[start_idx] = 0.0;

    let mut prev: Vec<Option<usize>> = vec![None; n];

    let mut pq: BinaryHeap<DistNode> = BinaryHeap::new();
    pq.push(DistNode {
      dist: 0.0,
      node: start_idx,
    });

    let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for edge in &self.edges {
      if let (Some(&from_idx), Some(&to_idx)) = (
        node_index.get(edge.from.as_str()),
        node_index.get(edge.to.as_str()),
      ) {
        adj[from_idx].push((to_idx, edge.weight));
        adj[to_idx].push((from_idx, edge.weight));
      }
    }

    while let Some(DistNode { dist: d, node: u }) = pq.pop() {
      if d > dist[u] {
        continue;
      }
      if u == end_idx {
        break;
      }
      for &(v, w) in &adj[u] {
        let new_dist = dist[u] + w;
        if new_dist < dist[v] {
          dist[v] = new_dist;
          prev[v] = Some(u);
          pq.push(DistNode {
            dist: new_dist,
            node: v,
          });
        }
      }
    }

    if dist[end_idx] == f64::INFINITY {
      return None;
    }

    let mut path = Vec::new();
    let mut curr = Some(end_idx);
    while let Some(idx) = curr {
      path.push(node_ids[idx].to_string());
      curr = prev[idx];
    }
    path.reverse();

    Some((path, dist[end_idx]))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_graph_new() {
    let g = Graph::new();
    assert!(g.nodes.is_empty());
    assert!(g.edges.is_empty());
  }

  #[test]
  fn test_graph_add_node() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({"name": "Node A"}));
    g.add_node("B", serde_json::json!({"name": "Node B"}));
    assert_eq!(g.nodes.len(), 2);
  }

  #[test]
  fn test_graph_add_edge() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({}));
    g.add_node("B", serde_json::json!({}));
    g.add_edge("A", "B", 1.0);
    assert_eq!(g.edges.len(), 1);
    assert_eq!(g.edges[0].weight, 1.0);
  }

  #[test]
  fn test_dijkstra_simple() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({}));
    g.add_node("B", serde_json::json!({}));
    g.add_node("C", serde_json::json!({}));
    g.add_edge("A", "B", 1.0);
    g.add_edge("B", "C", 2.0);
    g.add_edge("A", "C", 5.0);

    let result = g.dijkstra_shortest_path("A", "C");
    assert!(result.is_some());
    let (path, dist) = result.unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);
    assert_eq!(dist, 3.0);
  }

  #[test]
  fn test_dijkstra_no_path() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({}));
    g.add_node("B", serde_json::json!({}));
    assert!(g.dijkstra_shortest_path("A", "B").is_none());
  }

  #[test]
  fn test_dijkstra_same_node() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({}));
    let result = g.dijkstra_shortest_path("A", "A");
    assert!(result.is_some());
    let (path, dist) = result.unwrap();
    assert_eq!(path, vec!["A"]);
    assert_eq!(dist, 0.0);
  }

  #[test]
  fn test_dijkstra_missing_node() {
    let mut g = Graph::new();
    g.add_node("A", serde_json::json!({}));
    g.add_node("B", serde_json::json!({}));
    assert!(g.dijkstra_shortest_path("A", "C").is_none());
  }
}
