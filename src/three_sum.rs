use std::collections::HashMap;
struct Solution;

impl Solution {
    // find all combinations of 3 numsbers in nums where i != j, j!= k, and k != i, and
    // where nums[i] + nums[k] + nums[j] == 0;
    // numbers can repeat, just not indices
    // combinatorial problem
    // n choose 3
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut three_sums: Vec<Vec<i32>> = Vec::new();

        let n = nums.len();

        // Calculate the result iteratively

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i != j && i != k && j != k {
                        let sum = nums[i] + nums[j] + nums[k];
                        if sum == 0 {
                            three_sums.push(vec![nums[i], nums[j], nums[k]]);
                        }
                    }
                }
            }
        }

        three_sums.dedup();
        three_sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinatorics::choose;
    #[test]
    fn six_nums_success() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let res = Solution::three_sum(nums);

        assert_eq!(res, output);
    }
}
