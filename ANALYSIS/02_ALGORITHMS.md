# tauri-shared Algorithms

## algorithms-manifest.json Contents
```json
{
  "algorithms": [
    {"name": "quick_sort", "category": "sorting", "time_complexity": "O(n log n) avg, O(n^2) worst", "space": "O(log n)", "stable": false},
    {"name": "merge_sort", "category": "sorting", "time_complexity": "O(n log n)", "space": "O(n)", "stable": true},
    {"name": "bubble_sort", "category": "sorting", "time_complexity": "O(n^2)", "space": "O(1)", "stable": true},
    {"name": "insertion_sort", "category": "sorting", "time_complexity": "O(n^2)", "space": "O(1)", "stable": true},
    {"name": "dijkstra", "category": "graph", "time_complexity": "O((V+E) log V)", "space": "O(V)", "stable": true}
  ],
  "sorting_algorithms": ["quick_sort", "merge_sort", "bubble_sort", "insertion_sort"],
  "graph_algorithms": ["dijkstra"]
}
```

## Implementations

### Sorting (src/algorithms/sorting.rs)
Generic implementations accepting `T: Ord + Clone`:
- `quick_sort()` - middle-element pivot selection
- `merge_sort()` - top-down divide and conquer
- `bubble_sort()` - standard comparison sort
- `insertion_sort()` - adaptive O(n^2)

### Graph (src/algorithms/graph.rs)
Requires `algorithms` feature (petgraph dependency):
- `create_graph()` - creates directed graph
- `add_node()` - adds node with label
- `add_edge()` - adds weighted edge
- `dijkstra_shortest_path()` - shortest path from start node

### JSON-Aware Commands (src/commands/algorithm_commands.rs)
Tauri commands with JSON serialization:
- `json_quick_sort()` / `quick_sort` command
- `json_merge_sort()` / `merge_sort` command
- `json_bubble_sort()` / `bubble_sort` command
- `json_insertion_sort()` / `insertion_sort` command
- `dijkstra()` / `dijkstra` command

Includes custom `json_ord()` function for comparing serde_json::Value types.

## Feature Gate
```toml
[features]
algorithms = ["dep:petgraph"]
```
