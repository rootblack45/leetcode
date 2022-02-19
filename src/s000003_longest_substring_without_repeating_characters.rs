use std::collections::HashMap;

impl super::Solution {
    /// Given a string `s`, find the length of the **longest substring** without repeating characters.
    ///
    /// ## Example
    ///
    /// ```text
    /// Input: s = "abcabcbb"
    /// Output: 3
    /// Explanation: The answer is "abc", with the length of 3.
    /// ```
    /// ```text
    /// Input: s = "bbbbb"
    /// Output: 1
    /// Explanation: The answer is "b", with the length of 1.
    /// ```
    /// ```text
    /// Input: s = "pwwkew"
    /// Output: 3
    /// Explanation: The answer is "wke", with the length of 3.
    /// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    /// ```
    ///
    /// ## Constraints
    ///
    /// - `0 <= s.length <= 5 * 10^4`
    /// - `s` consists of English letters, digits, symbols and spaces.
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = HashMap::new();
        let mut a = 0;
        let mut b = -1;
        for (i, c) in s.chars().enumerate() {
            if let Some(l) = m.insert(c, i) {
                b = b.max(l as i32)
            }
            a = a.max(i as i32 - b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn example_test_cases() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    }
}
