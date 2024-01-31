use lazy_static::lazy_static;
use std::fmt;
use std::sync::Mutex;

// Define a custom error for invalid input
#[derive(Debug)]
pub struct CombinatorialError {
    message: String,
}

impl CombinatorialError {
    fn new(msg: &str) -> CombinatorialError {
        CombinatorialError {
            message: msg.to_string(),
        }
    }
}
#[derive(Debug)]
pub struct PermutationError {
    message: String,
}

impl PermutationError {
    fn new(msg: &str) -> PermutationError {
        PermutationError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CombinatorialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl fmt::Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CombinatorialError {}
impl std::error::Error for PermutationError {}

lazy_static! {
    static ref FACTORIAL_CACHE: Mutex<Vec<u128>> = Mutex::new(vec![1, 1]); // 0! = 1, 1! = 1
}

pub fn factorial(n: u128) -> u128 {
    {
        let cache = FACTORIAL_CACHE.lock().unwrap();

        // Check if the result is already in the cache
        if n < cache.len() as u128 {
            return cache[n as usize];
        }
    } // Release the lock before the calculation

    // Start from the largest factorial we know
    let mut result = {
        let cache = FACTORIAL_CACHE.lock().unwrap();
        cache.last().unwrap().clone()
    };

    let start = {
        let cache = FACTORIAL_CACHE.lock().unwrap();
        cache.len()
    };

    for i in start..=n as usize {
        result *= i as u128;
        let mut cache = FACTORIAL_CACHE.lock().unwrap();
        cache.push(result);
    }

    result
}

pub fn choose_unoptimized(n: u128, k: u128) -> Result<u128, CombinatorialError> {
    if n < k {
        return Err(CombinatorialError::new(
            format!("n must be greater than or equal to k - n {}, k: {}", n, k).as_str(),
        ));
    }

    Ok(factorial(n) / (factorial(k) * factorial(n - k)))
}
// nCr = n!/k!(n-k)!
pub fn choose(n: u128, k: u128) -> Result<u128, CombinatorialError> {
    if n < k {
        return Err(CombinatorialError::new(
            format!("n must be greater than or equal to k - n {}, k: {}", n, k).as_str(),
        ));
    }
    // Check for negative k is not needed as u128 is always non-negative

    // Optimize by calculating with the smaller of k and n-k
    let k = std::cmp::min(k, n - k);

    // Calculate the result iteratively
    let mut result: u128 = 1;
    for i in 1..=k as usize {
        result = result * (n - k + i as u128) / i as u128;
    }

    Ok(result)
}
// nPr = n!/(n-k)!
pub fn permutation(n: u128, k: u128) -> Result<u128, &'static str> {
    if n < k {
        return Err("n must be greater than or equal to k.");
    }

    let mut result: u128 = 1;
    for i in 0..k {
        result = result
            .checked_mul(n - i)
            .ok_or("Overflow occurred in permutation calculation")?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_five_success() {
        let num = 5;
        let output: u128 = 120;
        let res = factorial(num);
        assert_eq!(res, output);
    }
    #[test]
    fn five_choose_two() {
        let n = 5;
        let k = 2;
        let output: u128 = 10;
        let res = choose_unoptimized(n, k).unwrap_or(0);
        let res1 = choose(n, k).unwrap_or(0);
        assert_eq!((res), output);
        assert_eq!(res1, output);
    }
    #[test]
    fn twenty_choose_seven() {
        let n = 20;
        let k = 7;
        let output: u128 = 77520;
        let res = choose_unoptimized(n, k).unwrap_or(0);
        let res1 = choose(n, k).unwrap_or(0);
        assert_eq!(res, output);
        assert_eq!(res1, output);
    }
    #[test]
    fn one_hundred_choose_fifty() {
        let n = 100;
        let k = 50;
        let output: u128 = 100891344545564193334812497256;
        let res = choose(n, k).unwrap_or(0);
        assert_eq!(res, output);
    }
    #[test]
    fn five_permutations_of_ten() {
        let n = 10;
        let k = 5;
        let output: u128 = 30240;
        let res = permutation(n, k).unwrap_or(0);
        assert_eq!(res, output);
    }
}
