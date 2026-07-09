//! Math algorithms and statistics (Rust port of `tauri-front-shared/algorithms/math.ts`).

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }
  a
}

pub fn lcm(a: u64, b: u64) -> u64 {
  if a == 0 || b == 0 {
    0
  } else {
    a / gcd(a, b) * b
  }
}

pub fn factorial(n: u64) -> u128 {
  if n <= 1 {
    1
  } else {
    (2..=n).map(|i| i as u128).product()
  }
}

pub fn is_prime(n: u64) -> bool {
  if n < 2 {
    return false;
  }
  if n < 4 {
    return true;
  }
  if n % 2 == 0 {
    return false;
  }
  let mut i = 3;
  while i * i <= n {
    if n % i == 0 {
      return false;
    }
    i += 2;
  }
  true
}

/// Sieve of Eratosthenes. Returns all primes <= n.
pub fn primes_up_to(n: u64) -> Vec<u64> {
  if n < 2 {
    return vec![];
  }
  let n = n as usize;
  let mut sieve = vec![false; n + 1];
  let mut primes = Vec::new();
  for i in 2..=n {
    if sieve[i] {
      continue;
    }
    primes.push(i as u64);
    let mut j = i * i;
    while j <= n {
      sieve[j] = true;
      j += i;
    }
  }
  primes
}

pub fn fibonacci(n: u64) -> u128 {
  if n == 0 {
    return 0;
  }
  let (mut a, mut b) = (0u128, 1u128);
  for _ in 2..=n {
    let t = a + b;
    a = b;
    b = t;
  }
  b
}

/// Fast exponentiation. O(log exp).
pub fn power(base: u64, exp: u32) -> u64 {
  let mut result = 1u64;
  let mut b = base;
  let mut e = exp;
  while e > 0 {
    if e & 1 == 1 {
      result *= b;
    }
    b *= b;
    e >>= 1;
  }
  result
}

pub fn is_power_of_two(n: u64) -> bool {
  n > 0 && (n & (n - 1)) == 0
}

pub fn next_power_of_two(n: u64) -> u64 {
  if n <= 1 {
    return 1;
  }
  let mut p = 1u64;
  while p < n {
    p <<= 1;
  }
  p
}

pub fn mean(nums: &[f64]) -> f64 {
  if nums.is_empty() {
    return 0.0;
  }
  nums.iter().sum::<f64>() / nums.len() as f64
}

pub fn median(nums: &mut [f64]) -> f64 {
  if nums.is_empty() {
    return 0.0;
  }
  nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let n = nums.len();
  if n % 2 == 0 {
    (nums[n / 2 - 1] + nums[n / 2]) / 2.0
  } else {
    nums[n / 2]
  }
}

pub fn stddev(nums: &[f64]) -> f64 {
  if nums.len() < 2 {
    return 0.0;
  }
  let m = mean(nums);
  let s: f64 = nums.iter().map(|x| (x - m).powi(2)).sum();
  (s / (nums.len() as f64 - 1.0)).sqrt()
}
