use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::algorithms::{
  graph::{Graph, GraphEdge, GraphNode},
  SearchAlgorithm, ValidationAlgorithm,
};

pub type AlgorithmFn =
  Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>;

#[derive(Default, Clone)]
pub struct AlgorithmRegistry {
  algorithms: Arc<RwLock<HashMap<String, AlgorithmFn>>>,
}

impl AlgorithmRegistry {
  pub fn new() -> Self {
    let registry = Self {
      algorithms: Arc::new(RwLock::new(HashMap::new())),
    };
    registry.register_builtins();
    registry
  }

  pub fn register(&self, name: String, func: AlgorithmFn) {
    let mut algorithms = self.algorithms.write().unwrap();
    algorithms.insert(name, func);
  }

  pub fn execute(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value, String> {
    let algorithms = self.algorithms.read().unwrap();
    let func = algorithms
      .get(name)
      .ok_or_else(|| format!("Algorithm not found: {name}"))?;
    func(input)
  }

  pub fn list(&self) -> Vec<String> {
    let algorithms = self.algorithms.read().unwrap();
    algorithms.keys().cloned().collect()
  }

  fn register_builtins(&self) {
    // Sorting algorithms — use json_sort with serde_json::Value-aware comparison
    self.register(
      "sort.bubble".to_string(),
      Box::new(|input| {
        let mut data: Vec<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        json_bubble_sort(&mut data);
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "sort.insertion".to_string(),
      Box::new(|input| {
        let mut data: Vec<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        json_insertion_sort(&mut data);
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "sort.merge".to_string(),
      Box::new(|input| {
        let mut data: Vec<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        json_merge_sort(&mut data);
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "sort.quick".to_string(),
      Box::new(|input| {
        let mut data: Vec<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        json_quick_sort(&mut data);
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    // Search algorithms
    self.register(
      "search.schemas".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SearchInput<T> {
          items: Vec<T>,
          query: String,
        }
        let input: SearchInput<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        let strings: Vec<String> = input
          .items
          .iter()
          .filter_map(|v| v.as_str().map(|s| s.to_string()))
          .collect();
        let result = SearchAlgorithm::search_schemas(&strings, &input.query);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "search.paginate".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct PaginateInput<T> {
          items: Vec<T>,
          page: u64,
          limit: u64,
        }
        let input: PaginateInput<serde_json::Value> =
          serde_json::from_value(input).map_err(|e| e.to_string())?;
        let result = SearchAlgorithm::paginate(&input.items, input.page, input.limit);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // Graph algorithms
    self.register(
      "graph.dijkstra".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct DijkstraInput {
          nodes: Vec<GraphNode>,
          edges: Vec<GraphEdge>,
          start: String,
          end: String,
        }
        let input: DijkstraInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut graph = Graph::new();
        for node in input.nodes {
          graph.add_node(&node.id, node.data);
        }
        for edge in input.edges {
          graph.add_edge(&edge.from, &edge.to, edge.weight);
        }
        let result = graph.dijkstra_shortest_path(&input.start, &input.end);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // Validation algorithms
    self.register(
      "validate.email".to_string(),
      Box::new(|input| {
        let email: String = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let result = ValidationAlgorithm::validate_email(&email);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "validate.input".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct ValidateInput {
          input: String,
          max_length: usize,
        }
        let input: ValidateInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let result = ValidationAlgorithm::validate_input(&input.input, input.max_length);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    self.register(
      "validate.sanitize".to_string(),
      Box::new(|input| {
        let input_str: String = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let result = ValidationAlgorithm::sanitize_input(&input_str);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );
  }
}

/// JSON value ordering: Null < Bool < Number < String < Array < Object
fn json_ord(a: &serde_json::Value, b: &serde_json::Value) -> std::cmp::Ordering {
  use serde_json::Value;
  match (a, b) {
    (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
    (Value::Null, _) => std::cmp::Ordering::Less,
    (_, Value::Null) => std::cmp::Ordering::Greater,
    (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
    (Value::Bool(_), _) => std::cmp::Ordering::Less,
    (_, Value::Bool(_)) => std::cmp::Ordering::Greater,
    (Value::Number(an), Value::Number(bn)) => {
      // Compare as f64 for numeric ordering
      match (an.as_f64(), bn.as_f64()) {
        (Some(a_f), Some(b_f)) => a_f.partial_cmp(&b_f).unwrap_or(std::cmp::Ordering::Equal),
        _ => an.to_string().cmp(&bn.to_string()),
      }
    }
    (Value::Number(_), _) => std::cmp::Ordering::Less,
    (_, Value::Number(_)) => std::cmp::Ordering::Greater,
    (Value::String(as_str), Value::String(bs_str)) => as_str.cmp(bs_str),
    (Value::String(_), _) => std::cmp::Ordering::Less,
    (_, Value::String(_)) => std::cmp::Ordering::Greater,
    (Value::Array(_), Value::Array(_)) => {
      // Compare element by element
      let a_arr = a.as_array().unwrap();
      let b_arr = b.as_array().unwrap();
      for (av, bv) in a_arr.iter().zip(b_arr.iter()) {
        let ord = json_ord(av, bv);
        if ord != std::cmp::Ordering::Equal {
          return ord;
        }
      }
      a_arr.len().cmp(&b_arr.len())
    }
    (Value::Array(_), _) => std::cmp::Ordering::Less,
    (_, Value::Array(_)) => std::cmp::Ordering::Greater,
    (Value::Object(_), Value::Object(_)) => {
      // Compare as sorted string representation
      a.to_string().cmp(&b.to_string())
    }
  }
}

// JSON-aware bubble sort
fn json_bubble_sort(data: &mut [serde_json::Value]) {
  let n = data.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
      if json_ord(&data[j], &data[j + 1]) == std::cmp::Ordering::Greater {
        data.swap(j, j + 1);
      }
    }
  }
}

// JSON-aware insertion sort
fn json_insertion_sort(data: &mut [serde_json::Value]) {
  for i in 1..data.len() {
    let mut j = i;
    while j > 0 && json_ord(&data[j - 1], &data[j]) == std::cmp::Ordering::Greater {
      data.swap(j - 1, j);
      j -= 1;
    }
  }
}

// JSON-aware merge sort
fn json_merge_sort(data: &mut [serde_json::Value]) {
  if data.len() <= 1 {
    return;
  }
  let mid = data.len() / 2;
  let mut left = data[..mid].to_vec();
  let mut right = data[mid..].to_vec();
  json_merge_sort(&mut left);
  json_merge_sort(&mut right);
  json_merge(data, &left, &right);
}

fn json_merge(
  result: &mut [serde_json::Value],
  left: &[serde_json::Value],
  right: &[serde_json::Value],
) {
  let mut i = 0;
  let mut j = 0;
  let mut k = 0;
  while i < left.len() && j < right.len() {
    if json_ord(&left[i], &right[j]) != std::cmp::Ordering::Greater {
      result[k] = left[i].clone();
      i += 1;
    } else {
      result[k] = right[j].clone();
      j += 1;
    }
    k += 1;
  }
  while i < left.len() {
    result[k] = left[i].clone();
    i += 1;
    k += 1;
  }
  while j < right.len() {
    result[k] = right[j].clone();
    j += 1;
    k += 1;
  }
}

// JSON-aware quick sort
fn json_quick_sort(data: &mut [serde_json::Value]) {
  if data.is_empty() {
    return;
  }
  json_quick_sort_impl(data, 0, data.len() - 1);
}

fn json_quick_sort_impl(data: &mut [serde_json::Value], low: usize, high: usize) {
  if low < high {
    let pivot_idx = json_partition(data, low, high);
    if pivot_idx > 0 {
      json_quick_sort_impl(data, low, pivot_idx - 1);
    }
    if pivot_idx < high {
      json_quick_sort_impl(data, pivot_idx + 1, high);
    }
  }
}

fn json_partition(data: &mut [serde_json::Value], low: usize, high: usize) -> usize {
  let pivot = data[high].clone();
  let mut i = low;
  for j in low..high {
    if json_ord(&data[j], &pivot) != std::cmp::Ordering::Greater {
      data.swap(i, j);
      i += 1;
    }
  }
  data.swap(i, high);
  i
}
