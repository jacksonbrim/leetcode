use std::collections::HashMap;
/// Return the indices of the two numbers such that they add up to target
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<usize>> {
    for (idx, num) in nums.iter().enumerate() {
        for (idx2, num2) in nums.iter().enumerate() {
            if idx != idx2 && (num + num2 == target) {
                return Some(vec![idx, idx2]);
            }
        }
    }
    return None;
}
pub fn two_sum_perf(nums: Vec<i32>, target: i32) -> Option<Vec<usize>> {
    let mut seen = HashMap::with_capacity(nums.len()); // Create a hash map to store numbers and their indices

    for (idx, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_idx) = seen.get(&complement) {
            // If the complement is found in the map, return the indices
            return Some(vec![complement_idx, idx]);
        }
        // Store the current number and its index in the map
        seen.insert(num, idx);
    }
    return None;
}

pub fn two_sum_as_i32(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, num) in nums.iter().enumerate() {
        for (idx2, num2) in nums.iter().enumerate() {
            if idx != idx2 && (num + num2 == target) {
                return vec![idx as i32, idx2 as i32];
            }
        }
    }
    panic!("No two sum solution found!");
}

pub fn two_sum_as_i32_perf(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::with_capacity(nums.len()); // Create a hash map to store numbers and their indices

    for (idx, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_idx) = seen.get(&complement) {
            // If the complement is found in the map, return the indices
            return vec![complement_idx as i32, idx as i32];
        }
        // Store the current number and its index in the map
        seen.insert(num, idx);
    }
    panic!("No two sum solution found!");
}

pub fn main() {
    // target = 4, result should be [0, 1]
    let target_1: i32 = 4;
    let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
    let res_1 = two_sum_perf(test_case1.clone(), target_1).unwrap();
    println!(
        "{} + {} = {}",
        test_case1[res_1[0]], test_case1[res_1[1]], target_1
    );

    // target = 4, result should be [0, 1]
    let target2: i32 = 4;
    let test_case2: Vec<i32> = vec![1, 3, 9, 4, 13];
    let res2 = two_sum_as_i32_perf(test_case2.clone(), target2);
    println!(
        "{} + {} = {}",
        test_case2[res2[0] as usize], test_case2[res2[1] as usize], target2
    );
}

#[cfg(test)]
mod tests {
    use crate::two_sum::*;
    #[test]
    fn first_two_success() {
        let target_1: i32 = 4;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let ans_1: Vec<usize> = vec![0, 1];
        let ans_i32_1: Vec<i32> = vec![0, 1];
        let res_1 = two_sum(test_case1.clone(), target_1);
        let perf_res_1 = two_sum_perf(test_case1.clone(), target_1);
        let res_i32_1 = two_sum_as_i32(test_case1.clone(), target_1);
        let perf_res_i32_1 = two_sum_as_i32_perf(test_case1.clone(), target_1);

        assert_eq!(res_1.unwrap(), ans_1);
        assert_eq!(perf_res_1.unwrap(), ans_1);
        assert_eq!(res_i32_1, ans_i32_1);
        assert_eq!(perf_res_i32_1, ans_i32_1);
    }

    #[test]
    fn first_last_success() {
        let target_1: i32 = 14;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let ans_1: Vec<usize> = vec![0, 4];
        let ans_i32_1: Vec<i32> = vec![0, 4];
        let res_1 = two_sum(test_case1.clone(), target_1);
        let perf_res_1 = two_sum_perf(test_case1.clone(), target_1);
        let res_i32_1 = two_sum_as_i32(test_case1.clone(), target_1);
        let perf_res_i32_1 = two_sum_as_i32_perf(test_case1.clone(), target_1);

        assert_eq!(res_1.unwrap(), ans_1);
        assert_eq!(perf_res_1.unwrap(), ans_1);
        assert_eq!(res_i32_1, ans_i32_1);
        assert_eq!(perf_res_i32_1, ans_i32_1);
    }

    #[test]
    fn last_two_success() {
        let target_1: i32 = 17;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let ans_1: Vec<usize> = vec![3, 4];
        let ans_i32_1: Vec<i32> = vec![3, 4];
        let res_1 = two_sum(test_case1.clone(), target_1);
        let perf_res_1 = two_sum_perf(test_case1.clone(), target_1);
        let res_i32_1 = two_sum_as_i32(test_case1.clone(), target_1);
        let perf_res_i32_1 = two_sum_as_i32_perf(test_case1.clone(), target_1);

        assert_eq!(res_1.unwrap(), ans_1);
        assert_eq!(perf_res_1.unwrap(), ans_1);
        assert_eq!(res_i32_1, ans_i32_1);
        assert_eq!(perf_res_i32_1, ans_i32_1);
    }

    #[test]
    fn no_sum_matches_target_no_panic() {
        let target_1: i32 = 0;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let res_1 = two_sum(test_case1.clone(), target_1);
        let perf_res_1 = two_sum_perf(test_case1.clone(), target_1);

        assert_eq!(res_1, None);
        assert_eq!(perf_res_1, None);
    }

    #[test]
    #[should_panic(expected = "No two sum solution found!")]
    fn no_sum_matches_target_i32_panic() {
        let target_1: i32 = 0;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let _ = two_sum_as_i32(test_case1.clone(), target_1);
    }

    #[test]
    #[should_panic(expected = "No two sum solution found!")]
    fn no_sum_matches_target_i32_perf_panic() {
        let target_1: i32 = 0;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let _ = two_sum_as_i32_perf(test_case1.clone(), target_1);
    }

    #[test]
    fn highest_sum() {
        let target_1: i32 = 22;
        let test_case1: Vec<i32> = vec![1, 3, 9, 4, 13];
        let ans_1: Vec<usize> = vec![2, 4];
        let ans_i32_1: Vec<i32> = vec![2, 4];
        let res_1 = two_sum(test_case1.clone(), target_1);
        let perf_res_1 = two_sum_perf(test_case1.clone(), target_1);
        let res_i32_1 = two_sum_as_i32(test_case1.clone(), target_1);
        let perf_res_i32_1 = two_sum_as_i32_perf(test_case1.clone(), target_1);

        assert_eq!(res_1.unwrap(), ans_1);
        assert_eq!(perf_res_1.unwrap(), ans_1);
        assert_eq!(res_i32_1, ans_i32_1);
        assert_eq!(perf_res_i32_1, ans_i32_1);
    }
}
