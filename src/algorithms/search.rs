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