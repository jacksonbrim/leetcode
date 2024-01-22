pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let arr_len1 = nums1.len();
    let arr_len2 = nums2.len();
    let med_loc = arr_len1 + arr_len2 / 2;

    let mut med1: f64 = 0.;
    let mut med2: f64 = 0.;
    let mut med: f64 = 0.;
    if (arr_len1 + arr_len2) % 2 == 0 {
        let med_loc = (arr_len1 + arr_len2 / 2) - 1;
    } else {
    }
}

#[cfg(test)]
mod tests {
    use crate::median_sorted_arrays::*;

    #[test]
    fn success() {
        let nums1: Vec<i32> = vec![1, 3];
        let nums2: Vec<i32> = vec![2];
        let output1: f64 = 2.;

        let res1 = find_median_sorted_arrays(nums1, nums2);

        assert_eq!(res1, output1);
    }
}
