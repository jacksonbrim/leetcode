use std::collections::HashSet;
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

        let mut nums = nums;
        nums.sort_by(|a, b| a.cmp(b));
        let mut i_nums = Vec::new();
        let mut j_nums = Vec::new();
        let mut k_nums = Vec::new();
        let mut seen_three_sum: HashSet<Vec<i32>> = HashSet::new();

        let mut skipped_k = 0;
        'i_loop: for i in 0..n {
            let a = nums[i];
            let i_seen = i_nums.contains(&a);
            if i_seen {
                k_nums.clear();
                j_nums.clear();
                continue 'i_loop;
            }
            i_nums.push(a);
            'j_loop: for j in i..n {
                let b = nums[j];
                if i == j {
                    k_nums.clear();
                    continue 'j_loop;
                }
                j_nums.push(b);
                'k_loop: for k in j..n {
                    let c = nums[k];

                    let k_seen = k_nums.contains(&c);
                    if k_seen || (k == j || k == i) {
                        skipped_k += 1;
                        continue 'k_loop;
                    }
                    k_nums.push(c);
                    let sum = a + b + c;
                    if sum == 0 {
                        let three_sum = vec![a, b, c];
                        let _ = !seen_three_sum.insert(three_sum);
                    }
                }
                k_nums.clear();
            }
            j_nums.clear();
        }
        three_sums = seen_three_sum.drain().collect();
        three_sums.sort_by(|a, b| a.cmp(b));
        three_sums
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

    //#[test]
    //fn random_nums_pos_and_neg_success() {
    //    let nums = vec![
    //        -11, -3, -6, 12, -15, -13, -7, -3, 13, -2, -10, 3, 12, -12, 6, -6, 12, 9, -2, -12, 14,
    //        11, -4, 11, -8, 8, 0, -12, 4, -5, 10, 8, 7, 11, -3, 7, 5, -3, -11, 3, 11, -13, 14, 8,
    //        12, 5, -12, 10, -8, -7, 5, -9, -11, -14, 9, -12, 1, -6, -8, -10, 4, 9, 6, -3, -3, -12,
    //        11, 9, 1, 8, -10, -3, 2, -11, -10, -1, 1, -15, -6, 8, -7, 6, 6, -10, 7, 0, -7, -7, 9,
    //        -8, -9, -9, -14, 12, -5, -10, -15, -9, -15, -7, 6, -10, 5, -7, -14, 3, 8, 2, 3, 9, -12,
    //        4, 1, 9, 1, -15, -13, 9, -14, 11, 9,
    //    ];
    //    let mut output: Vec<Vec<i32>> = vec![vec![0]];
    //    output.sort_by(|a, b| a.cmp(b));
    //    let res = Solution::three_sum(nums);

    //    assert_eq!(res, output);
    //}
}
