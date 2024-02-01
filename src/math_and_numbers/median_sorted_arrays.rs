pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_length = nums1.len() + nums2.len();
        let mut p1 = 0;
        let mut p2 = 0;
        let mut last_val = 0;
        let mut current_val = 0;

        for _ in 0..=(total_length / 2) {
            last_val = current_val;
            if p1 < nums1.len() && (p2 >= nums2.len() || nums1[p1] <= nums2[p2]) {
                current_val = nums1[p1];
                p1 += 1;
            } else {
                current_val = nums2[p2];
                p2 += 1;
            }
        }

        if total_length % 2 == 0 {
            (current_val + last_val) as f64 / 2.0
        } else {
            current_val as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success1() {
        let nums1: Vec<i32> = vec![1, 3];
        let nums2: Vec<i32> = vec![2];
        let output1: f64 = 2.;

        let res1 = Solution::find_median_sorted_arrays(nums1, nums2);

        assert_eq!(res1, output1);
    }
    #[test]
    fn success2() {
        let nums1: Vec<i32> = vec![1, 2];
        let nums2: Vec<i32> = vec![3, 4];
        let output1: f64 = 2.5;

        let res1 = Solution::find_median_sorted_arrays(nums1, nums2);

        assert_eq!(res1, output1);
    }
    #[test]
    fn success3() {
        let nums1: Vec<i32> = vec![1, 2, 3, 3, 3, 4, 5, 8, 9, 12];
        let nums2: Vec<i32> = vec![2, 2, 2, 2, 3, 4, 4, 8, 9, 12];
        let output1: f64 = 3.5;

        let res1 = Solution::find_median_sorted_arrays(nums1, nums2);

        assert_eq!(res1, output1);
    }
}
