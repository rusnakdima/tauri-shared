//! String algorithms (Rust port of `tauri-front-shared/algorithms/string.ts`).

/// Levenshtein edit distance. O(m*n) time, O(min(m,n)) space.
pub fn levenshtein(a: &str, b: &str) -> usize {
  if a == b {
    return 0;
  }
  let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) };
  let m = a.len();
  let n = b.len();
  if m == 0 {
    return n;
  }
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();
  let mut prev: Vec<usize> = (0..=n).collect();
  let mut curr: Vec<usize> = vec![0; n + 1];
  for i in 1..=m {
    curr[0] = i;
    for j in 1..=n {
      let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
        0
      } else {
        1
      };
      curr[j] = (curr[j - 1] + 1).min(prev[j] + 1).min(prev[j - 1] + cost);
    }
    std::mem::swap(&mut prev, &mut curr);
  }
  prev[n]
}

/// Hamming distance. Returns `None` if strings have different lengths.
pub fn hamming(a: &str, b: &str) -> Option<usize> {
  if a.len() != b.len() {
    return None;
  }
  Some(a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count())
}

/// Longest common subsequence length.
pub fn lcs_length(a: &str, b: &str) -> usize {
  let a = a.as_bytes();
  let b = b.as_bytes();
  let m = a.len();
  let n = b.len();
  if m == 0 || n == 0 {
    return 0;
  }
  let mut prev: Vec<usize> = vec![0; n + 1];
  let mut curr: Vec<usize> = vec![0; n + 1];
  for i in 1..=m {
    for j in 1..=n {
      curr[j] = if a[i - 1] == b[j - 1] {
        prev[j - 1] + 1
      } else {
        curr[j - 1].max(prev[j])
      };
    }
    std::mem::swap(&mut prev, &mut curr);
  }
  prev[n]
}

pub fn is_palindrome(s: &str) -> bool {
  let bytes = s.as_bytes();
  let n = bytes.len();
  for i in 0..n / 2 {
    if bytes[i] != bytes[n - 1 - i] {
      return false;
    }
  }
  true
}
