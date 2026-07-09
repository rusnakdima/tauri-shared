use std::cmp::Ordering;

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return;
  }
  let pivot_idx = arr.len() / 2;
  arr.swap(pivot_idx, arr.len() - 1);
  let mut pivot_final_idx = 0;
  for i in 0..arr.len() - 1 {
    if arr[i] < arr[arr.len() - 1] {
      arr.swap(i, pivot_final_idx);
      pivot_final_idx += 1;
    }
  }
  arr.swap(pivot_final_idx, arr.len() - 1);
  let (left, right) = arr.split_at_mut(pivot_final_idx);
  quick_sort(left);
  quick_sort(&mut right[1..]);
}

pub fn quick_sort_by<T>(arr: &mut [T], cmp: &dyn Fn(&T, &T) -> Ordering) {
  if arr.len() <= 1 {
    return;
  }
  let pivot_idx = arr.len() / 2;
  arr.swap(pivot_idx, arr.len() - 1);
  let mut pivot_final_idx = 0;
  for i in 0..arr.len() - 1 {
    if cmp(&arr[i], &arr[arr.len() - 1]) == Ordering::Less {
      arr.swap(i, pivot_final_idx);
      pivot_final_idx += 1;
    }
  }
  arr.swap(pivot_final_idx, arr.len() - 1);
  let (left, right) = arr.split_at_mut(pivot_final_idx);
  quick_sort_by(left, cmp);
  quick_sort_by(&mut right[1..], cmp);
}

pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
  if arr.len() <= 1 {
    return arr.to_vec();
  }
  let mid = arr.len() / 2;
  let left = merge_sort(&arr[..mid]);
  let right = merge_sort(&arr[mid..]);
  merge(&left, &right)
}

pub fn merge_sort_by<T: Clone>(arr: &[T], cmp: &dyn Fn(&T, &T) -> Ordering) -> Vec<T> {
  if arr.len() <= 1 {
    return arr.to_vec();
  }
  let mid = arr.len() / 2;
  let left = merge_sort_by(&arr[..mid], cmp);
  let right = merge_sort_by(&arr[mid..], cmp);
  merge_by(&left, &right, cmp)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
  merge_by(left, right, |a, b| a.cmp(b))
}

fn merge_by<T: Clone>(left: &[T], right: &[T], cmp: impl Fn(&T, &T) -> Ordering) -> Vec<T> {
  let mut result = Vec::with_capacity(left.len() + right.len());
  let mut i = 0;
  let mut j = 0;
  while i < left.len() && j < right.len() {
    if cmp(&left[i], &right[j]) != Ordering::Greater {
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

pub fn bubble_sort_by<T>(arr: &mut [T], cmp: &dyn Fn(&T, &T) -> Ordering) {
  let n = arr.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
      if cmp(&arr[j], &arr[j + 1]) == Ordering::Greater {
        arr.swap(j, j + 1);
      }
    }
  }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && arr[j - 1] > arr[j] {
      arr.swap(j - 1, j);
      j -= 1;
    }
  }
}

pub fn insertion_sort_by<T>(arr: &mut [T], cmp: &dyn Fn(&T, &T) -> Ordering) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && cmp(&arr[j - 1], &arr[j]) == Ordering::Greater {
      arr.swap(j - 1, j);
      j -= 1;
    }
  }
}

// JSON-aware wrappers (require algorithms feature)
#[cfg(feature = "algorithms")]
pub mod json {
  use super::*;
  use serde_json::Value;

  pub fn json_ord(a: &Value, b: &Value) -> Ordering {
    use serde_json::Value::*;
    match (a, b) {
      (Null, Null) => Ordering::Equal,
      (Null, _) => Ordering::Less,
      (_, Null) => Ordering::Greater,
      (Bool(false), Bool(false)) => Ordering::Equal,
      (Bool(false), _) => Ordering::Less,
      (_, Bool(false)) => Ordering::Greater,
      (Bool(true), Bool(true)) => Ordering::Equal,
      (Bool(true), _) => Ordering::Less,
      (_, Bool(true)) => Ordering::Greater,
      (Number(na), Number(nb)) => {
        let (Some(aa), Some(bb)) = (na.as_f64(), nb.as_f64()) else {
          return Ordering::Equal;
        };
        aa.partial_cmp(&bb).unwrap_or(Ordering::Equal)
      }
      (Number(_), _) => Ordering::Less,
      (_, Number(_)) => Ordering::Greater,
      (String(sa), String(sb)) => sa.cmp(sb),
      (String(_), _) => Ordering::Less,
      (_, String(_)) => Ordering::Greater,
      (Array(_), Array(_)) => Ordering::Equal,
      (Array(_), _) => Ordering::Less,
      (_, Array(_)) => Ordering::Greater,
      (Object(_), Object(_)) => Ordering::Equal,
    }
  }

  pub fn quick_sort(arr: &mut [Value]) {
    super::quick_sort_by(arr, &json_ord);
  }

  pub fn merge_sort(arr: &[Value]) -> Vec<Value> {
    super::merge_sort_by(arr, &json_ord)
  }

  pub fn bubble_sort(arr: &mut [Value]) {
    super::bubble_sort_by(arr, &json_ord);
  }

  pub fn insertion_sort(arr: &mut [Value]) {
    super::insertion_sort_by(arr, &json_ord);
  }
}
