pub fn quick_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let pivot = arr[arr.len() / 2].clone();
    let left: Vec<T> = arr.iter().filter(|x| *x < &pivot).cloned().collect();
    let middle: Vec<T> = arr.iter().filter(|x| *x == &pivot).cloned().collect();
    let right: Vec<T> = arr.iter().filter(|x| *x > &pivot).cloned().collect();
    let mut result = quick_sort(&left);
    result.extend(middle);
    result.extend(quick_sort(&right));
    result
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

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
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

pub fn bubble_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut result = arr.to_vec();
    let n = result.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }
    result
}

pub fn insertion_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut result = arr.to_vec();
    for i in 1..result.len() {
        let key = result[i].clone();
        let mut j = i;
        while j > 0 && result[j - 1] > key {
            result[j] = result[j - 1].clone();
            j -= 1;
        }
        result[j] = key;
    }
    result
}
