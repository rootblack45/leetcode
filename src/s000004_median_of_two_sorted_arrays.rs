impl super::Solution {
    /// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays.
    ///
    /// The overall run time complexity should be `O(log (m+n))`.
    ///
    /// ## Example
    ///
    /// ```text
    /// Input: nums1 = [1,3], nums2 = [2]
    /// Output: 2.00000
    /// Explanation: merged array = [1,2,3] and median is 2.
    /// ```
    /// ```text
    /// Input: nums1 = [1,2], nums2 = [3,4]
    /// Output: 2.50000
    /// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
    /// ```
    ///
    /// ## Constraints
    ///
    /// - `nums1.length == m`
    /// - `nums2.length == n`
    /// - `0 <= m <= 1000`
    /// - `0 <= n <= 1000`
    /// - `1 <= m + n <= 2000`
    /// - `-10^6 <= nums1[i], nums2[i] <= 10^6`
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = [nums1, nums2].concat();
        nums.sort_unstable();
        let len = nums.len();
        if len % 2 == 0 {
            (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0
        } else {
            nums[len / 2] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn example_test_cases() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2, 3]), 2.5);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]),
            1.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 3], vec![1]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3], vec![-2, -1]),
            -1.0
        );
    }
}
