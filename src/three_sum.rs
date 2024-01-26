use std::collections::HashMap;
struct Solution;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FACTORIAL_CACHE: Mutex<Vec<u64>> = Mutex::new(vec![1, 1]); // 0! = 1, 1! = 1
}

impl Solution {
    // find all combinations of 3 numsbers in nums where i != j, j!= k, and k != i, and
    // where nums[i] + nums[k] + nums[j] == 0;
    // numbers can repeat, just not indices
    // combinatorial problem
    // n choose 3
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut three_sums: Vec<Vec<i32>> = Vec::new();
        let combinations = nums.len();
        for i in 0..nums.len() {
            for j in i..nums.len() {
                for k in j..nums.len() {
                    if i != j && j != k && k != i {
                        dbg!(i, j, k);
                        let three_sum = nums[i] + nums[j] + nums[k];
                        if three_sum == 0 {
                            three_sums.push(vec![nums[i], nums[j], nums[k]]);
                        }
                    }
                }
            }
        }
        three_sums
    }
    fn factorial(n: usize) -> u64 {
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
            result *= i as u64;
            let mut cache = FACTORIAL_CACHE.lock().unwrap();
            cache.push(result);
        }

        result
    }

    pub fn choose(n: usize, k: usize) -> u64 {
        (Self::factorial(n) as f64 / (Self::factorial(k) as f64 * Self::factorial(n - k) as f64))
            as u64
    }
    pub fn choose2(n: usize, k: usize) -> u64 {
        if n > k {
            (Self::factorial(n - k) as f64 / Self::factorial(n - k) as f64) as u64
        } else if k > n {
            (1. / (Self::factorial(k - n) as f64) * Self::factorial(n - k) as f64) as u64
        } else {
            (1 as f64 / Self::factorial(n - k) as f64) as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn six_nums_success() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let res = Solution::three_sum(nums);
        assert_eq!(res, output);
    }
    #[test]
    fn factorial_five_success() {
        let num = 5;
        let output: u64 = 120;
        let res = Solution::factorial(num);
        assert_eq!(res, output);
    }
    #[test]
    fn five_choose_two() {
        let n = 5;
        let k = 2;
        let output: u64 = 10;
        let res = Solution::choose(n, k);
        //let res1 = Solution::choose2(n, k);
        assert_eq!(res, output);
        //assert_eq!(res1, output);
    }
    #[test]
    fn twenty_choose_seven() {
        let n = 20;
        let k = 7;
        let output: u64 = 77520;
        let res = Solution::choose(n, k);
        //let res1 = Solution::choose2(n, k);
        assert_eq!(res, output);
        //assert_eq!(res1, output);
    }
}
