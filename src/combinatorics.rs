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

impl fmt::Display for CombinatorialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CombinatorialError {}

lazy_static! {
    static ref FACTORIAL_CACHE: Mutex<Vec<usize>> = Mutex::new(vec![1, 1]); // 0! = 1, 1! = 1
}

pub fn factorial(n: usize) -> usize {
    {
        let cache = FACTORIAL_CACHE.lock().unwrap();

        // Check if the result is already in the cache
        if n < cache.len() {
            return cache[n];
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

    for i in start..=n {
        result *= i;
        let mut cache = FACTORIAL_CACHE.lock().unwrap();
        cache.push(result);
    }

    result
}

pub fn choose_unoptimized(n: usize, k: usize) -> Result<usize, CombinatorialError> {
    if n < k {
        return Err(CombinatorialError::new(
            format!("n must be greater than or equal to k - n {}, k: {}", n, k).as_str(),
        ));
    }

    Ok(factorial(n) / (factorial(k) * factorial(n - k)))
}
pub fn choose(n: usize, k: usize) -> Result<usize, CombinatorialError> {
    if n < k {
        return Err(CombinatorialError::new(
            format!("n must be greater than or equal to k - n {}, k: {}", n, k).as_str(),
        ));
    }
    // Check for negative k is not needed as usize is always non-negative

    // Optimize by calculating with the smaller of k and n-k
    let k = std::cmp::min(k, n - k);

    // Calculate the result iteratively
    let mut result: usize = 1;
    for i in 1..=k {
        result = result * (n - k + i) / i;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_five_success() {
        let num = 5;
        let output: usize = 120;
        let res = factorial(num);
        assert_eq!(res, output);
    }
    #[test]
    fn five_choose_two() {
        let n = 5;
        let k = 2;
        let output: usize = 10;
        let res = choose_unoptimized(n, k).unwrap_or(0);
        let res1 = choose(n, k).unwrap_or(0);
        assert_eq!((res), output);
        assert_eq!(res1, output);
    }
    #[test]
    fn twenty_choose_seven() {
        let n = 20;
        let k = 7;
        let output: usize = 77520;
        let res = choose_unoptimized(n, k).unwrap_or(0);
        let res1 = choose(n, k).unwrap_or(0);
        assert_eq!(res, output);
        assert_eq!(res1, output);
    }
}
