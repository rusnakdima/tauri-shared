use std::time::{Duration, Instant};

pub struct LruCache<K, V> {
    capacity: usize,
    entries: Vec<(K, V, Instant)>,
}

impl<K: Eq + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .iter()
            .find(|(k, _, _)| k == key)
            .map(|(_, v, _)| v)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(pos) = self.entries.iter().position(|(k, _, _)| k == &key) {
            self.entries[pos].1 = value;
            self.entries[pos].2 = Instant::now();
        } else {
            if self.entries.len() >= self.capacity {
                self.evict_lru();
            }
            self.entries.push((key, value, Instant::now()));
        }
    }

    pub fn evict_lru(&mut self) -> Option<(K, V)> {
        if self.entries.is_empty() {
            return None;
        }
        let min_index = self
            .entries
            .iter()
            .enumerate()
            .min_by_key(|(_, (_, _, time))| *time)
            .map(|(idx, _)| idx);
        min_index.map(|idx| {
            let (k, v, _) = self.entries.remove(idx);
            (k, v)
        })
    }

    pub fn is_stale(&self, key: &K, max_age_ms: u64) -> bool {
        let max_age = Duration::from_millis(max_age_ms);
        self.entries
            .iter()
            .find(|(k, _, _)| k == key)
            .map(|(_, _, time)| time.elapsed() > max_age)
            .unwrap_or(true)
    }
}
