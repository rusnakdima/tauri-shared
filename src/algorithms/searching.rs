//! Searching algorithms (Rust port of `tauri-front-shared/algorithms/searching.ts`).

/// Linear search. Returns the index of the first match, or `None`.
pub fn linear_search<T, F: Fn(&T) -> bool>(arr: &[T], pred: F) -> Option<usize> {
  arr.iter().position(|x| pred(x))
}

/// Binary search on a sorted slice. Returns the index, or `None`.
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
  arr.binary_search(target).ok()
}

/// First index where `arr[i] >= target` (sorted slice).
pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
  arr
    .binary_search_by(|x| {
      if x < target {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    })
    .unwrap_or_else(|e| e)
}

/// First index where `arr[i] > target` (sorted slice).
pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
  arr
    .binary_search_by(|x| {
      if x <= target {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    })
    .unwrap_or_else(|e| e)
}

/// Jump search on a sorted slice. O(√n).
pub fn jump_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
  let n = arr.len();
  if n == 0 {
    return None;
  }
  let step = ((n as f64).sqrt() as usize).max(1);
  let mut prev = 0;
  while prev < n && arr[std::cmp::min(step, n) - 1] < *target {
    prev += step;
    if prev >= n {
      return None;
    }
  }
  for i in prev..std::cmp::min(prev + step, n) {
    if &arr[i] == target {
      return Some(i);
    }
  }
  None
}

use std::cmp::Ordering;
