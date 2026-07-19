use std::cmp::Ordering;

/// Sorts a slice using the bubble sort algorithm.
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
  let n = arr.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }
}

/// Sorts a slice using the bubble sort algorithm with a custom comparator.
pub fn bubble_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
  F: FnMut(&T, &T) -> Ordering,
{
  let n = arr.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
      if compare(&arr[j], &arr[j + 1]) == Ordering::Greater {
        arr.swap(j, j + 1);
      }
    }
  }
}

/// Sorts a slice using the insertion sort algorithm.
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && arr[j - 1] > arr[j] {
      arr.swap(j - 1, j);
      j -= 1;
    }
  }
}

/// Sorts a slice using the insertion sort algorithm with a custom comparator.
pub fn insertion_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
  F: FnMut(&T, &T) -> Ordering,
{
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && compare(&arr[j - 1], &arr[j]) == Ordering::Greater {
      arr.swap(j - 1, j);
      j -= 1;
    }
  }
}

/// Sorts a slice using the merge sort algorithm.
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return;
  }
  let mid = arr.len() / 2;
  let mut left = arr[..mid].to_vec();
  let mut right = arr[mid..].to_vec();
  merge_sort(&mut left);
  merge_sort(&mut right);
  merge(arr, &left, &right);
}

fn merge<T: Ord + Clone>(result: &mut [T], left: &[T], right: &[T]) {
  let mut i = 0;
  let mut j = 0;
  let mut k = 0;
  while i < left.len() && j < right.len() {
    if left[i] <= right[j] {
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

/// Sorts a slice using the merge sort algorithm with a custom comparator.
pub fn merge_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
  F: FnMut(&T, &T) -> Ordering + Clone,
  T: Clone,
{
  if arr.len() <= 1 {
    return;
  }
  let mid = arr.len() / 2;
  let mut left: Vec<T> = arr[..mid].to_vec();
  let mut right: Vec<T> = arr[mid..].to_vec();
  let compare_left = compare.clone();
  let compare_right = compare.clone();
  merge_sort_by(&mut left, compare_left);
  merge_sort_by(&mut right, compare_right);
  merge_by(arr, &left, &right, &mut compare);
}

fn merge_by<T, F>(result: &mut [T], left: &[T], right: &[T], compare: &mut F)
where
  F: FnMut(&T, &T) -> Ordering,
  T: Clone,
{
  let mut i = 0;
  let mut j = 0;
  let mut k = 0;
  while i < left.len() && j < right.len() {
    if compare(&left[i], &right[j]) != Ordering::Greater {
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

/// Sorts a slice using the quick sort algorithm.
pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  quick_sort_impl(arr, 0, arr.len() - 1);
}

fn quick_sort_impl<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
  if low < high {
    let pivot_idx = partition(arr, low, high);
    if pivot_idx > 0 {
      quick_sort_impl(arr, low, pivot_idx - 1);
    }
    if pivot_idx < high {
      quick_sort_impl(arr, pivot_idx + 1, high);
    }
  }
}

fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
  let pivot = arr[high].clone();
  let mut i = low;
  for j in low..high {
    if arr[j] <= pivot {
      arr.swap(i, j);
      i += 1;
    }
  }
  arr.swap(i, high);
  i
}

/// Sorts a slice using the quick sort algorithm with a custom comparator.
pub fn quick_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
  F: FnMut(&T, &T) -> Ordering + Clone,
  T: Clone,
{
  if arr.is_empty() {
    return;
  }
  quick_sort_by_impl(arr, 0, arr.len() - 1, &mut compare);
}

fn quick_sort_by_impl<T, F>(arr: &mut [T], low: usize, high: usize, compare: &mut F)
where
  F: FnMut(&T, &T) -> Ordering,
  T: Clone,
{
  if low < high {
    let pivot_idx = partition_by(arr, low, high, compare);
    if pivot_idx > 0 {
      quick_sort_by_impl(arr, low, pivot_idx - 1, compare);
    }
    if pivot_idx < high {
      quick_sort_by_impl(arr, pivot_idx + 1, high, compare);
    }
  }
}

fn partition_by<T, F>(arr: &mut [T], low: usize, high: usize, compare: &mut F) -> usize
where
  F: FnMut(&T, &T) -> Ordering,
  T: Clone,
{
  let pivot = arr[high].clone();
  let mut i = low;
  for j in low..high {
    if compare(&arr[j], &pivot) != Ordering::Greater {
      arr.swap(i, j);
      i += 1;
    }
  }
  arr.swap(i, high);
  i
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bubble_sort() {
    let mut v = vec![5, 2, 8, 1, 9];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 5, 8, 9]);
  }

  #[test]
  fn test_bubble_sort_by() {
    let mut v = vec![5, 2, 8, 1, 9];
    bubble_sort_by(&mut v, |a, b| b.cmp(a));
    assert_eq!(v, vec![9, 8, 5, 2, 1]);
  }

  #[test]
  fn test_bubble_sort_empty() {
    let mut v: Vec<i32> = vec![];
    bubble_sort(&mut v);
    assert!(v.is_empty());
  }

  #[test]
  fn test_bubble_sort_single() {
    let mut v = vec![1];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1]);
  }

  #[test]
  fn test_insertion_sort() {
    let mut v = vec![5, 2, 8, 1, 9];
    insertion_sort(&mut v);
    assert_eq!(v, vec![1, 2, 5, 8, 9]);
  }

  #[test]
  fn test_insertion_sort_by() {
    let mut v = vec![5, 2, 8, 1, 9];
    insertion_sort_by(&mut v, |a, b| b.cmp(a));
    assert_eq!(v, vec![9, 8, 5, 2, 1]);
  }

  #[test]
  fn test_insertion_sort_empty() {
    let mut v: Vec<i32> = vec![];
    insertion_sort(&mut v);
    assert!(v.is_empty());
  }

  #[test]
  fn test_insertion_sort_single() {
    let mut v = vec![1];
    insertion_sort(&mut v);
    assert_eq!(v, vec![1]);
  }

  #[test]
  fn test_merge_sort() {
    let mut v = vec![5, 2, 8, 1, 9];
    merge_sort(&mut v);
    assert_eq!(v, vec![1, 2, 5, 8, 9]);
  }

  #[test]
  fn test_merge_sort_by() {
    let mut v = vec![5, 2, 8, 1, 9];
    merge_sort_by(&mut v, |a, b| b.cmp(a));
    assert_eq!(v, vec![9, 8, 5, 2, 1]);
  }

  #[test]
  fn test_merge_sort_empty() {
    let mut v: Vec<i32> = vec![];
    merge_sort(&mut v);
    assert!(v.is_empty());
  }

  #[test]
  fn test_merge_sort_single() {
    let mut v = vec![1];
    merge_sort(&mut v);
    assert_eq!(v, vec![1]);
  }

  #[test]
  fn test_quick_sort() {
    let mut v = vec![5, 2, 8, 1, 9];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 5, 8, 9]);
  }

  #[test]
  fn test_quick_sort_by() {
    let mut v = vec![5, 2, 8, 1, 9];
    quick_sort_by(&mut v, |a, b| b.cmp(a));
    assert_eq!(v, vec![9, 8, 5, 2, 1]);
  }

  #[test]
  fn test_quick_sort_empty() {
    let mut v: Vec<i32> = vec![];
    quick_sort(&mut v);
    assert!(v.is_empty());
  }

  #[test]
  fn test_quick_sort_single() {
    let mut v = vec![1];
    quick_sort(&mut v);
    assert_eq!(v, vec![1]);
  }

  #[test]
  fn test_quick_sort_already_sorted() {
    let mut v = vec![1, 2, 3, 4, 5];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_quick_sort_reverse_sorted() {
    let mut v = vec![5, 4, 3, 2, 1];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
  }
}
