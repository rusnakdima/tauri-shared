#[cfg(feature = "algorithms")]
use std::collections::HashMap;

#[cfg(feature = "algorithms")]
use petgraph::{graph::Graph, graph::NodeIndex, Directed};

#[cfg(feature = "algorithms")]
use crate::algorithms::sorting::json;

#[cfg(feature = "algorithms")]
fn json_ord(a: &serde_json::Value, b: &serde_json::Value) -> std::cmp::Ordering {
  json::json_ord(a, b)
}

#[cfg(feature = "algorithms")]
fn json_quick_sort(arr: &mut [serde_json::Value]) {
  json::quick_sort(arr);
}

#[cfg(feature = "algorithms")]
fn json_merge_sort(arr: &[serde_json::Value]) -> Vec<serde_json::Value> {
  json::merge_sort(arr)
}

#[cfg(feature = "algorithms")]
fn json_bubble_sort(arr: &mut [serde_json::Value]) {
  json::bubble_sort(arr);
}

#[cfg(feature = "algorithms")]
fn json_insertion_sort(arr: &mut [serde_json::Value]) {
  json::insertion_sort(arr);
}

#[tauri::command]
#[cfg(feature = "algorithms")]
pub fn quick_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
  let mut arr = arr;
  json_quick_sort(&mut arr);
  Ok(arr)
}

#[tauri::command]
#[cfg(feature = "algorithms")]
pub fn merge_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
  Ok(json_merge_sort(&arr))
}

#[tauri::command]
#[cfg(feature = "algorithms")]
pub fn bubble_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
  let mut arr = arr;
  json_bubble_sort(&mut arr);
  Ok(arr)
}

#[tauri::command]
#[cfg(feature = "algorithms")]
pub fn insertion_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
  let mut arr = arr;
  json_insertion_sort(&mut arr);
  Ok(arr)
}

#[cfg(feature = "algorithms")]
fn build_graph(
  adj_list: &HashMap<String, Vec<(String, f64)>>,
) -> (Graph<String, f64, Directed>, HashMap<String, NodeIndex>) {
  let mut graph = Graph::new();
  let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

  for node in adj_list.keys() {
    node_indices.insert(node.clone(), graph.add_node(node.clone()));
  }

  for (node, neighbors) in adj_list {
    let from_idx = node_indices[node];
    for (neighbor, weight) in neighbors {
      if let Some(to_idx) = node_indices.get(neighbor) {
        graph.add_edge(from_idx, *to_idx, *weight);
      }
    }
  }

  (graph, node_indices)
}

#[tauri::command]
#[cfg(feature = "algorithms")]
pub fn dijkstra(
  adj_list: HashMap<String, Vec<(String, f64)>>,
  start: String,
) -> Result<HashMap<String, Option<f64>>, String> {
  use petgraph::algo::dijkstra;

  let (graph, node_indices) = build_graph(&adj_list);

  let start_idx = node_indices
    .get(&start)
    .ok_or_else(|| format!("Start node '{}' not found in graph", start))?;

  let distances = dijkstra(&graph, *start_idx, None, |e| *e.weight());

  let result: HashMap<String, Option<f64>> = node_indices
    .iter()
    .map(|(name, idx)| {
      let dist = distances.get(idx).copied();
      (name.clone(), dist)
    })
    .collect();

  Ok(result)
}
