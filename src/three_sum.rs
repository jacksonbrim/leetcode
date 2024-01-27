use std::collections::HashSet;
struct Solution;

impl Solution {
    // find all combinations of 3 numsbers in nums where i != j, j!= k, and k != i, and
    // where nums[i] + nums[k] + nums[j] == 0;
    // numbers can repeat, just not indices
    // combinatorial problem
    // n choose 3
    pub fn naive_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut three_sums: Vec<Vec<i32>> = Vec::new();

        let n = nums.len();

        let mut nums = nums;
        nums.sort();
        let mut i_nums = Vec::new();
        let mut j_nums = Vec::new();
        let mut k_nums = Vec::new();
        let mut seen_three_sum: HashSet<Vec<i32>> = HashSet::new();

        'i_loop: for i in 0..n {
            let a = nums[i];
            let i_seen = i_nums.contains(&a);
            j_nums.clear();
            k_nums.clear();
            if i_seen {
                continue 'i_loop;
            }
            i_nums.push(a);
            'j_loop: for j in i..n {
                let b = nums[j];
                k_nums.clear();
                if i == j {
                    continue 'j_loop;
                }
                j_nums.push(b);
                'k_loop: for k in j..n {
                    let c = nums[k];

                    let k_seen = k_nums.contains(&c);
                    if k_seen || (k == j || k == i) {
                        continue 'k_loop;
                    }
                    k_nums.push(c);
                    let sum = a + b + c;
                    if sum == 0 {
                        let three_sum = vec![a, b, c];
                        let _ = !seen_three_sum.insert(three_sum);
                    }
                }
            }
        }
        three_sums = seen_three_sum.drain().collect();
        three_sums
    }

    // two pointers method
    // 27ms runtime, beats 94.36%, 3.92MB Memory beats 86.11%
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut three_sums: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            // skip the repeats
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    three_sums.push(vec![nums[i], nums[left], nums[right]]);
                    // Skip duplicates for left and right
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        three_sums
    }

    // 26ms, beats 96.38% of rust users, 4.12MB memory beating 43.99% of users with rust
    pub fn three_sum_perf(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort(); // Sort the array to use two-pointer technique

        for i in 0..nums.len() {
            // Avoid duplicates for the first element
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    // Move left and right pointers and avoid duplicates for them
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn six_nums_success() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut output: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        output.sort_by(|a, b| a.cmp(b));
        let res = Solution::three_sum(nums);

        assert_eq!(res, output);
    }
    #[test]
    fn all_zeros_success() {
        let nums = vec![0, 0, 0, 0, 0, 0, 0];
        let mut output: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        output.sort_by(|a, b| a.cmp(b));
        let res = Solution::three_sum(nums);

        assert_eq!(res, output);
    }

    #[test]
    fn random_nums_pos_and_neg_success() {
        let nums = vec![
            -11, -3, -6, 12, -15, -13, -7, -3, 13, -2, -10, 3, 12, -12, 6, -6, 12, 9, -2, -12, 14,
            11, -4, 11, -8, 8, 0, -12, 4, -5, 10, 8, 7, 11, -3, 7, 5, -3, -11, 3, 11, -13, 14, 8,
            12, 5, -12, 10, -8, -7, 5, -9, -11, -14, 9, -12, 1, -6, -8, -10, 4, 9, 6, -3, -3, -12,
            11, 9, 1, 8, -10, -3, 2, -11, -10, -1, 1, -15, -6, 8, -7, 6, 6, -10, 7, 0, -7, -7, 9,
            -8, -9, -9, -14, 12, -5, -10, -15, -9, -15, -7, 6, -10, 5, -7, -14, 3, 8, 2, 3, 9, -12,
            4, 1, 9, 1, -15, -13, 9, -14, 11, 9,
        ];
        let mut output = vec![
            vec![-15, 1, 14],
            vec![-15, 2, 13],
            vec![-15, 3, 12],
            vec![-15, 4, 11],
            vec![-15, 5, 10],
            vec![-15, 6, 9],
            vec![-15, 7, 8],
            vec![-14, 0, 14],
            vec![-14, 1, 13],
            vec![-14, 2, 12],
            vec![-14, 3, 11],
            vec![-14, 4, 10],
            vec![-14, 5, 9],
            vec![-14, 6, 8],
            vec![-14, 7, 7],
            vec![-13, -1, 14],
            vec![-13, 0, 13],
            vec![-13, 1, 12],
            vec![-13, 2, 11],
            vec![-13, 3, 10],
            vec![-13, 4, 9],
            vec![-13, 5, 8],
            vec![-13, 6, 7],
            vec![-12, -2, 14],
            vec![-12, -1, 13],
            vec![-12, 0, 12],
            vec![-12, 1, 11],
            vec![-12, 2, 10],
            vec![-12, 3, 9],
            vec![-12, 4, 8],
            vec![-12, 5, 7],
            vec![-12, 6, 6],
            vec![-11, -3, 14],
            vec![-11, -2, 13],
            vec![-11, -1, 12],
            vec![-11, 0, 11],
            vec![-11, 1, 10],
            vec![-11, 2, 9],
            vec![-11, 3, 8],
            vec![-11, 4, 7],
            vec![-11, 5, 6],
            vec![-10, -4, 14],
            vec![-10, -3, 13],
            vec![-10, -2, 12],
            vec![-10, -1, 11],
            vec![-10, 0, 10],
            vec![-10, 1, 9],
            vec![-10, 2, 8],
            vec![-10, 3, 7],
            vec![-10, 4, 6],
            vec![-10, 5, 5],
            vec![-9, -5, 14],
            vec![-9, -4, 13],
            vec![-9, -3, 12],
            vec![-9, -2, 11],
            vec![-9, -1, 10],
            vec![-9, 0, 9],
            vec![-9, 1, 8],
            vec![-9, 2, 7],
            vec![-9, 3, 6],
            vec![-9, 4, 5],
            vec![-8, -6, 14],
            vec![-8, -5, 13],
            vec![-8, -4, 12],
            vec![-8, -3, 11],
            vec![-8, -2, 10],
            vec![-8, -1, 9],
            vec![-8, 0, 8],
            vec![-8, 1, 7],
            vec![-8, 2, 6],
            vec![-8, 3, 5],
            vec![-8, 4, 4],
            vec![-7, -7, 14],
            vec![-7, -6, 13],
            vec![-7, -5, 12],
            vec![-7, -4, 11],
            vec![-7, -3, 10],
            vec![-7, -2, 9],
            vec![-7, -1, 8],
            vec![-7, 0, 7],
            vec![-7, 1, 6],
            vec![-7, 2, 5],
            vec![-7, 3, 4],
            vec![-6, -6, 12],
            vec![-6, -5, 11],
            vec![-6, -4, 10],
            vec![-6, -3, 9],
            vec![-6, -2, 8],
            vec![-6, -1, 7],
            vec![-6, 0, 6],
            vec![-6, 1, 5],
            vec![-6, 2, 4],
            vec![-6, 3, 3],
            vec![-5, -5, 10],
            vec![-5, -4, 9],
            vec![-5, -3, 8],
            vec![-5, -2, 7],
            vec![-5, -1, 6],
            vec![-5, 0, 5],
            vec![-5, 1, 4],
            vec![-5, 2, 3],
            vec![-4, -3, 7],
            vec![-4, -2, 6],
            vec![-4, -1, 5],
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-4, 2, 2],
            vec![-3, -3, 6],
            vec![-3, -2, 5],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -2, 4],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-2, 1, 1],
            vec![-1, 0, 1],
        ];
        output.sort_by(|a, b| a.cmp(b));
        let res = Solution::three_sum(nums);

        assert_eq!(res, output);
    }
}
