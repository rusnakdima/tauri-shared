use std::collections::HashMap;

#[cfg(feature = "algorithms")]
use petgraph::{graph::Graph, graph::NodeIndex, Directed};

fn json_ord(a: &serde_json::Value, b: &serde_json::Value) -> std::cmp::Ordering {
    use serde_json::Value::*;
    match (a, b) {
        (Null, Null) => std::cmp::Ordering::Equal,
        (Null, _) => std::cmp::Ordering::Less,
        (_, Null) => std::cmp::Ordering::Greater,
        (Bool(false), Bool(false)) => std::cmp::Ordering::Equal,
        (Bool(false), _) => std::cmp::Ordering::Less,
        (_, Bool(false)) => std::cmp::Ordering::Greater,
        (Bool(true), Bool(true)) => std::cmp::Ordering::Equal,
        (Bool(true), _) => std::cmp::Ordering::Less,
        (_, Bool(true)) => std::cmp::Ordering::Greater,
        (Number(na), Number(nb)) => {
            let (Some(aa), Some(bb)) = (na.as_f64(), nb.as_f64()) else {
                return std::cmp::Ordering::Equal;
            };
            aa.partial_cmp(&bb).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Number(_), _) => std::cmp::Ordering::Less,
        (_, Number(_)) => std::cmp::Ordering::Greater,
        (String(sa), String(sb)) => sa.cmp(sb),
        (String(_), _) => std::cmp::Ordering::Less,
        (_, String(_)) => std::cmp::Ordering::Greater,
        (Array(_), Array(_)) => std::cmp::Ordering::Equal,
        (Array(_), _) => std::cmp::Ordering::Less,
        (_, Array(_)) => std::cmp::Ordering::Greater,
        (Object(_), Object(_)) => std::cmp::Ordering::Equal,
    }
}

fn json_quick_sort(arr: &[serde_json::Value]) -> Vec<serde_json::Value> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let pivot = arr[arr.len() / 2].clone();
    let pivot_ord = json_ord(&pivot, &pivot);
    let left: Vec<serde_json::Value> = arr
        .iter()
        .filter(|x| json_ord(x, &pivot) == std::cmp::Ordering::Less)
        .cloned()
        .collect();
    let middle: Vec<serde_json::Value> = arr
        .iter()
        .filter(|x| json_ord(x, &pivot) == pivot_ord)
        .cloned()
        .collect();
    let right: Vec<serde_json::Value> = arr
        .iter()
        .filter(|x| json_ord(x, &pivot) == std::cmp::Ordering::Greater)
        .cloned()
        .collect();
    let mut result = json_quick_sort(&left);
    result.extend(middle);
    result.extend(json_quick_sort(&right));
    result
}

fn json_merge_sort(arr: &[serde_json::Value]) -> Vec<serde_json::Value> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = json_merge_sort(&arr[..mid]);
    let right = json_merge_sort(&arr[mid..]);
    json_merge(&left, &right)
}

fn json_merge(left: &[serde_json::Value], right: &[serde_json::Value]) -> Vec<serde_json::Value> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if json_ord(&left[i], &right[j]) == std::cmp::Ordering::Less {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

#[tauri::command]
pub fn quick_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
    Ok(json_quick_sort(&arr))
}

#[tauri::command]
pub fn merge_sort(arr: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
    Ok(json_merge_sort(&arr))
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
pub fn dijkstra(
    adj_list: HashMap<String, Vec<(String, f64)>>,
    start: String,
) -> Result<HashMap<String, Option<f64>>, String> {
    #[cfg(feature = "algorithms")]
    {
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
    #[cfg(not(feature = "algorithms"))]
    {
        Err("algorithms feature not enabled".to_string())
    }
}
