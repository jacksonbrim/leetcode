struct Solution;
impl Solution {
    // 9ms, 72.48%, 2.26MB beating 32.11%
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();
        let n = nums.len();

        // convert to i64 to compare to sum
        let target = target as i64;

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicate
            }

            for j in i + 1..n {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue; // Skip duplicate
                }

                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    // sum must be an i64 to handle intermediate overflows
                    // however, as the target is initially an i32, the sum will always be within the
                    // bounds of an i32 should we push it to the Vec<i32> result.
                    let sum =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        while left < right && nums[left] == nums[left + 1] {
                            left += 1; // Skip duplicate
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1; // Skip duplicate
                        }

                        left += 1;
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        result
    }

    pub fn dfs_four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort(); // Sort the numbers to handle duplicates easily
        let mut result = Vec::new();
        Self::dfs(&nums, target, 0, &mut Vec::new(), &mut result);
        result
    }

    // depth first search
    fn dfs(
        nums: &[i32],
        target: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == 4 {
            // Compute the sum safely
            if let Some(sum) = current.iter().try_fold(0i32, |acc, &x| acc.checked_add(x)) {
                if sum == target {
                    result.push(current.clone()); // Found a valid combination
                }
            }
            return;
        }

        for i in start..nums.len() {
            // Skip duplicates
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }

            current.push(nums[i]);
            Self::dfs(nums, target, i + 1, current, result); // Recursively explore further
            current.pop(); // Backtrack
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_example1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let mut result = Solution::four_sum(nums, target);

        // Sort the inner vectors and the outer vector for comparison
        for vec in &mut result {
            vec.sort();
        }
        result.sort();

        let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_four_sum_example2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let mut result = Solution::four_sum(nums, target);

        // Sort the inner vectors and the outer vector for comparison
        for vec in &mut result {
            vec.sort();
        }
        result.sort();

        let expected = vec![vec![2, 2, 2, 2]];

        assert_eq!(result, expected);
    }
    #[test]
    fn test_four_sum_example3() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let mut result = Solution::four_sum(nums, target);

        // Sort the inner vectors and the outer vector for comparison
        for vec in &mut result {
            vec.sort();
        }
        result.sort();

        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result, expected);
    }
    #[test]
    fn four_sum_example_overflow_but_result_success() {
        let nums = vec![
            1000000000,
            1000000000,
            1000000000,
            1000000000,
            -1000000000,
            -1000000000,
            -1000000000,
            -1000000000,
        ];
        let target = 0;
        let mut result = Solution::four_sum(nums, target);

        // Sort the inner vectors and the outer vector for comparison
        for vec in &mut result {
            vec.sort();
        }
        result.sort();

        let expected: Vec<Vec<i32>> = vec![vec![-1000000000, -1000000000, 1000000000, 1000000000]];

        assert_eq!(result, expected);
    }
    // Function four_sum definition goes here
    // fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> { ... }
}
