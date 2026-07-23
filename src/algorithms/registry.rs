use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::algorithms::{
  graph::{Graph, GraphEdge, GraphNode},
  SearchAlgorithm, ValidationAlgorithm,
};

pub type AlgorithmFn =
  Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>;

/// Dynamically pluggable algorithm trait — implement this to add custom algorithms
/// to the registry without modifying the core library.
pub trait Algorithm: Send + Sync {
  fn name(&self) -> &str;
  fn domain(&self) -> &str;
  fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, String>;
  fn description(&self) -> &str;
}

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

  /// Register a function-based algorithm by name.
  pub fn register_fn(&self, name: String, func: AlgorithmFn) {
    let mut algorithms = self.algorithms.write().unwrap();
    algorithms.insert(name, func);
  }

  /// Register a dynamically pluggable algorithm via the `Algorithm` trait.
  pub fn register(&self, algo: Box<dyn Algorithm>) {
    let name = algo.name().to_string();
    let func: AlgorithmFn = Box::new(move |input| algo.execute(input));
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

  pub fn register_dyn(
    &mut self,
    name: String,
    handler: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>,
  ) {
    let mut algorithms = self.algorithms.write().unwrap();
    algorithms.insert(name, handler);
  }

  pub fn register_from(
    &mut self,
    algorithms: impl IntoIterator<
      Item = (
        String,
        Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>,
      ),
    >,
  ) {
    let mut inner = self.algorithms.write().unwrap();
    for (name, handler) in algorithms {
      inner.insert(name, handler);
    }
  }

  fn register_builtins(&self) {
    // Sorting algorithms — accept structured input { data: T[], field?: string, order?: 'asc' | 'desc' }
    // AlgorithmService sends { data: items, field, order } format
    self.register_fn(
      "sort.bubble".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SortInput {
          data: Vec<serde_json::Value>,
          #[serde(default)]
          field: Option<String>,
          #[serde(default)]
          order: Option<String>,
        }
        let input: SortInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut data = input.data;
        match &input.field {
          Some(field_str) => {
            let f = field_str.as_str();
            crate::algorithms::bubble_sort_by(&mut data, |a, b| {
              let va = extract_field_value(a, f);
              let vb = extract_field_value(b, f);
              json_ord(&va, &vb)
            });
          }
          None => {
            crate::algorithms::bubble_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register_fn(
      "sort.insertion".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SortInput {
          data: Vec<serde_json::Value>,
          #[serde(default)]
          field: Option<String>,
          #[serde(default)]
          order: Option<String>,
        }
        let input: SortInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut data = input.data;
        match &input.field {
          Some(field_str) => {
            let f = field_str.as_str();
            crate::algorithms::insertion_sort_by(&mut data, |a, b| {
              let va = extract_field_value(a, f);
              let vb = extract_field_value(b, f);
              json_ord(&va, &vb)
            });
          }
          None => {
            crate::algorithms::insertion_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register_fn(
      "sort.merge".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SortInput {
          data: Vec<serde_json::Value>,
          #[serde(default)]
          field: Option<String>,
          #[serde(default)]
          order: Option<String>,
        }
        let input: SortInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut data = input.data;
        match &input.field {
          Some(field_str) => {
            let f = field_str.as_str();
            crate::algorithms::merge_sort_by(&mut data, |a, b| {
              let va = extract_field_value(a, f);
              let vb = extract_field_value(b, f);
              json_ord(&va, &vb)
            });
          }
          None => {
            crate::algorithms::merge_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    self.register_fn(
      "sort.quick".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SortInput {
          data: Vec<serde_json::Value>,
          #[serde(default)]
          field: Option<String>,
          #[serde(default)]
          order: Option<String>,
        }
        let input: SortInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut data = input.data;
        match &input.field {
          Some(field_str) => {
            let f = field_str.as_str();
            crate::algorithms::quick_sort_by(&mut data, |a, b| {
              let va = extract_field_value(a, f);
              let vb = extract_field_value(b, f);
              json_ord(&va, &vb)
            });
          }
          None => {
            crate::algorithms::quick_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
        Ok(serde_json::to_value(data).map_err(|e| e.to_string())?)
      }),
    );

    // Search algorithms
    self.register_fn(
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

    self.register_fn(
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
    self.register_fn(
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

    // BFS — breadth-first search returning visited node ids in order
    self.register_fn(
      "graph.bfs".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct BfsInput {
          nodes: Vec<GraphNode>,
          edges: Vec<GraphEdge>,
          start: String,
        }
        let input: BfsInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut graph = Graph::new();
        for node in input.nodes {
          graph.add_node(&node.id, node.data);
        }
        for edge in input.edges {
          graph.add_edge(&edge.from, &edge.to, edge.weight);
        }
        let node_ids: Vec<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
        let mut adj: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
        for id in &node_ids {
          adj.entry(id).or_default();
        }
        for edge in &graph.edges {
          adj
            .entry(edge.from.as_str())
            .or_default()
            .push(edge.to.as_str());
          adj
            .entry(edge.to.as_str())
            .or_default()
            .push(edge.from.as_str());
        }
        let mut visited = std::collections::HashSet::new();
        let mut result: Vec<String> = Vec::new();
        let mut queue: std::collections::VecDeque<&str> =
          std::collections::VecDeque::from(vec![input.start.as_str()]);
        while let Some(node) = queue.pop_front() {
          if visited.contains(&node) {
            continue;
          }
          visited.insert(node);
          result.push(node.to_string());
          if let Some(neighbors) = adj.get(node) {
            for &n in neighbors {
              if !visited.contains(&n) {
                queue.push_back(n);
              }
            }
          }
        }
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // DFS — depth-first search returning visited node ids in order
    self.register_fn(
      "graph.dfs".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct DfsInput {
          nodes: Vec<GraphNode>,
          edges: Vec<GraphEdge>,
          start: String,
        }
        let input: DfsInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut graph = Graph::new();
        for node in input.nodes {
          graph.add_node(&node.id, node.data);
        }
        for edge in input.edges {
          graph.add_edge(&edge.from, &edge.to, edge.weight);
        }
        let node_ids: Vec<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
        let mut adj: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
        for id in &node_ids {
          adj.entry(id).or_default();
        }
        for edge in &graph.edges {
          adj
            .entry(edge.from.as_str())
            .or_default()
            .push(edge.to.as_str());
          adj
            .entry(edge.to.as_str())
            .or_default()
            .push(edge.from.as_str());
        }
        let mut visited = std::collections::HashSet::new();
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<&str> = vec![input.start.as_str()];
        while let Some(node) = stack.pop() {
          if visited.contains(&node) {
            continue;
          }
          visited.insert(node);
          result.push(node.to_string());
          if let Some(neighbors) = adj.get(node) {
            for &n in neighbors {
              if !visited.contains(&n) {
                stack.push(n);
              }
            }
          }
        }
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // Topological sort — Kahn's algorithm, returns nodes in dependency order
    self.register_fn(
      "graph.topological_sort".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct TopoInput {
          nodes: Vec<GraphNode>,
          edges: Vec<GraphEdge>,
        }
        let input: TopoInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut graph = Graph::new();
        for node in input.nodes {
          graph.add_node(&node.id, node.data);
        }
        for edge in &input.edges {
          graph.add_edge(&edge.from, &edge.to, edge.weight);
        }
        let mut in_degree: std::collections::HashMap<&str, usize> =
          std::collections::HashMap::new();
        for node in &graph.nodes {
          in_degree.entry(node.id.as_str()).or_insert(0);
        }
        let mut adj: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
        for id in graph.nodes.iter().map(|n| n.id.as_str()) {
          adj.entry(id).or_default();
        }
        for edge in &graph.edges {
          adj
            .entry(edge.from.as_str())
            .or_default()
            .push(edge.to.as_str());
          *in_degree.entry(edge.to.as_str()).or_insert(0) += 1;
        }
        let mut queue: Vec<&str> = Vec::new();
        for (id, &deg) in &in_degree {
          if deg == 0 {
            queue.push(id);
          }
        }
        let mut result: Vec<String> = Vec::new();
        while let Some(node) = queue.pop() {
          result.push(node.to_string());
          if let Some(neighbors) = adj.get(node) {
            for &n in neighbors {
              if let Some(d) = in_degree.get_mut(n) {
                *d -= 1;
                if *d == 0 {
                  queue.push(n);
                }
              }
            }
          }
        }
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // Tree algorithms
    self.register_fn(
      "tree.build".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct TreeNode {
          id: String,
          #[serde(default)]
          parent_id: Option<String>,
          #[serde(flatten)]
          data: serde_json::Value,
        }
        let items: Vec<TreeNode> = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut map: std::collections::HashMap<String, serde_json::Value> =
          std::collections::HashMap::new();
        let mut children_map: std::collections::HashMap<String, Vec<serde_json::Value>> =
          std::collections::HashMap::new();
        for item in &items {
          map.insert(item.id.clone(), item.data.clone());
          children_map.entry(item.id.clone()).or_default();
        }
        let mut roots: Vec<serde_json::Value> = Vec::new();
        for item in &items {
          if let Some(ref pid) = item.parent_id {
            if map.contains_key(pid) {
              let node = serde_json::json!({
                "id": item.id.clone(),
                "parentId": pid,
                "children": Vec::<serde_json::Value>::new(),
              });
              children_map.entry(pid.clone()).or_default().push(node);
            } else {
              let node = serde_json::json!({
                "id": item.id.clone(),
                "parentId": serde_json::Value::Null,
                "children": Vec::<serde_json::Value>::new(),
              });
              roots.push(node);
            }
          } else {
            let node = serde_json::json!({
              "id": item.id.clone(),
              "parentId": serde_json::Value::Null,
              "children": Vec::<serde_json::Value>::new(),
            });
            roots.push(node);
          }
        }
        // Attach children recursively
        fn attach_children(
          nodes: &mut [serde_json::Value],
          children_map: &std::collections::HashMap<String, Vec<serde_json::Value>>,
        ) {
          for node in nodes.iter_mut() {
            if let Some(id) = node.get("id").and_then(|v| v.as_str()) {
              if let Some(children) = children_map.get(id) {
                if let Some(obj) = node.as_object_mut() {
                  obj.insert("children".to_string(), serde_json::json!(children));
                  let mut slice = (*children).to_vec();
                  attach_children(&mut slice, children_map);
                  *obj.get_mut("children").unwrap() = serde_json::json!(slice);
                }
              }
            }
          }
        }
        attach_children(&mut roots, &children_map);
        Ok(serde_json::to_value(roots).map_err(|e| e.to_string())?)
      }),
    );

    self.register_fn(
      "tree.flatten".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum TreeNode {
          WithChildren { id: String, children: Vec<TreeNode> },
          Leaf { id: String },
        }
        fn flatten_nodes(nodes: &[TreeNode], result: &mut Vec<serde_json::Value>) {
          for node in nodes {
            let (id, children): (&str, Option<&[TreeNode]>) = match node {
              TreeNode::WithChildren { id, children } => (id.as_str(), Some(children.as_slice())),
              TreeNode::Leaf { id } => (id.as_str(), None),
            };
            let obj = serde_json::json!({
              "id": id,
              "children": Vec::<serde_json::Value>::new(),
            });
            result.push(obj);
            if let Some(c) = children {
              flatten_nodes(c, result);
            }
          }
        }
        let nodes: Vec<TreeNode> = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let mut result: Vec<serde_json::Value> = Vec::new();
        flatten_nodes(&nodes, &mut result);
        Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
      }),
    );

    // Validation algorithms
    self.register_fn(
      "validate.email".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct EmailInput {
          email: String,
        }
        let input: EmailInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let valid = ValidationAlgorithm::validate_email(&input.email);
        let result = if valid {
          serde_json::json!({ "valid": true })
        } else {
          serde_json::json!({ "valid": false, "errors": ["Invalid email format"] })
        };
        Ok(result)
      }),
    );

    self.register_fn(
      "validate.input".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct ValidateInput {
          input: String,
          max_length: usize,
        }
        let input: ValidateInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let valid = ValidationAlgorithm::validate_input(&input.input, input.max_length);
        let result = if valid {
          serde_json::json!({ "valid": true })
        } else {
          serde_json::json!({ "valid": false, "errors": ["Input invalid or empty"] })
        };
        Ok(result)
      }),
    );

    self.register_fn(
      "validate.sanitize".to_string(),
      Box::new(|input| {
        #[derive(serde::Deserialize)]
        struct SanitizeInput {
          input: String,
        }
        let input: SanitizeInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
        let result = ValidationAlgorithm::sanitize_input(&input.input);
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

// Extract field value from JSON object, returns self if not an object or field absent
fn extract_field_value(v: &serde_json::Value, field: &str) -> serde_json::Value {
  if let serde_json::Value::Object(obj) = v {
    obj.get(field).cloned().unwrap_or_else(|| v.clone())
  } else {
    v.clone()
  }
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
