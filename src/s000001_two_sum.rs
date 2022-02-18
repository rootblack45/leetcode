use std::collections::HashMap;

impl super::Solution {
    /// Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to target.
    /// You may assume that each input would have **exactly one solution**, and you may not use the same element twice.
    /// You can return the answer in any order.
    ///
    /// ## Example
    ///
    /// ```text
    /// Input: nums = [2,7,11,15], target = 9
    /// Output: [0,1]
    /// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    /// ```
    /// ```text
    /// Input: nums = [3,2,4], target = 6
    /// Output: [1,2]
    /// Explanation: Because nums[1] + nums[2] == 6, we return [1, 2].
    /// ```
    /// ```text
    /// Input: nums = [3,3], target = 6
    /// Output: [0,1]
    /// Explanation: Because nums[0] + nums[1] == 6, we return [0, 1].
    /// ```
    ///
    /// ## Constraints
    ///
    /// - `2 <= nums.length <= 10^4`
    /// - `-10^9 <= nums[i] <= 10^9`
    /// - `-10^9 <= target <= 10^9`
    /// - **Only one valid answer exists**.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = m.get(&(target - n)) {
                return vec![*j as i32, i as i32];
            }
            m.insert(n, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn example_test_cases() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
