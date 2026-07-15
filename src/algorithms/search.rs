#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search_schemas_empty_query() {
    let items = vec!["schema1", "schema2", "schema3"];
    let result = SearchAlgorithm::search_schemas(&items, "");
    assert_eq!(result.len(), 3);
  }

  #[test]
  fn test_search_schemas_exact_match() {
    let items = vec!["users", "products", "orders"];
    let result = SearchAlgorithm::search_schemas(&items, "users");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "users");
  }

  #[test]
  fn test_search_schemas_partial_match() {
    let items = vec!["user_profiles", "user_sessions", "products"];
    let result = SearchAlgorithm::search_schemas(&items, "user");
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"user_profiles"));
    assert!(result.contains(&"user_sessions"));
  }

  #[test]
  fn test_search_schemas_case_insensitive() {
    let items = vec!["Users", "PRODUCTS", "orders"];
    let result = SearchAlgorithm::search_schemas(&items, "users");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Users");
  }

  #[test]
  fn test_search_schemas_no_match() {
    let items = vec!["users", "products", "orders"];
    let result = SearchAlgorithm::search_schemas(&items, "xyz");
    assert!(result.is_empty());
  }

  #[test]
  fn test_search_schemas_with_struct() {
    #[derive(Clone, AsRef_str)]
    struct NamedItem {
        name: String,
    }
    let items = vec![
        NamedItem { name: "schema1".to_string() },
        NamedItem { name: "schema2".to_string() },
    ];
    let result = SearchAlgorithm::search_schemas(&items, "schema1");
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn test_paginate_first_page() {
    let items: Vec<i32> = (1..=10).collect();
    let result = SearchAlgorithm::paginate(&items, 1, 3);
    assert_eq!(result, vec![1, 2, 3]);
  }

  #[test]
  fn test_paginate_second_page() {
    let items: Vec<i32> = (1..=10).collect();
    let result = SearchAlgorithm::paginate(&items, 2, 3);
    assert_eq!(result, vec![4, 5, 6]);
  }

  #[test]
  fn test_paginate_last_page() {
    let items: Vec<i32> = (1..=10).collect();
    let result = SearchAlgorithm::paginate(&items, 4, 3);
    assert_eq!(result, vec![10]);
  }

  #[test]
  fn test_paginate_page_beyond_total() {
    let items: Vec<i32> = (1..=10).collect();
    let result = SearchAlgorithm::paginate(&items, 100, 3);
    assert!(result.is_empty());
  }

  #[test]
  fn test_paginate_page_zero() {
    let items: Vec<i32> = (1..=10).collect();
    let result = SearchAlgorithm::paginate(&items, 0, 3);
    // page 0 means start = 0 * 3 = 0 offset, so it returns first page
    assert_eq!(result, vec![1, 2, 3]);
  }

  #[test]
  fn test_paginate_limit_exceeds_items() {
    let items: Vec<i32> = (1..=3).collect();
    let result = SearchAlgorithm::paginate(&items, 1, 10);
    assert_eq!(result, vec![1, 2, 3]);
  }

  #[test]
  fn test_paginate_empty_slice() {
    let items: Vec<i32> = vec![];
    let result = SearchAlgorithm::paginate(&items, 1, 10);
    assert!(result.is_empty());
  }

  #[test]
  fn test_paginate_single_item_page() {
    let items: Vec<i32> = vec![42];
    let result = SearchAlgorithm::paginate(&items, 1, 1);
    assert_eq!(result, vec![42]);
  }
}

pub struct SearchAlgorithm;

impl SearchAlgorithm {
    pub fn search_schemas<T>(items: &[T], query: &str) -> Vec<T>
    where
        T: AsRef<str> + Clone,
    {
        let query = query.to_lowercase();
        items
            .iter()
            .filter(|s| s.as_ref().to_lowercase().contains(&query))
            .cloned()
            .collect()
    }

    pub fn paginate<T>(items: &[T], page: u64, limit: u64) -> Vec<T> {
        let start = ((page - 1) * limit) as usize;
        let end = start + limit as usize;

        if start < items.len() {
            items[start..items.len().min(end)].to_vec()
        } else {
            vec![]
        }
    }
}