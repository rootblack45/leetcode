use crate::utils::list_node::ListNode;

impl super::Solution {
    /// You are given two **non-empty** linked lists representing two non-negative integers. The digits are
    /// stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers
    /// and return the sum as a linked list.
    ///
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    ///
    /// ## Example
    ///
    /// ```text
    /// Input: l1 = [2,4,3], l2 = [5,6,4]
    /// Output: [7,0,8]
    /// Explanation: 342 + 465 = 807.
    /// ```
    /// ```text
    /// Input: l1 = [0], l2 = [0]
    /// Output: [0]
    /// ```
    /// ```text
    /// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    /// Output: [8,9,9,9,0,0,0,1]
    /// ```
    ///
    /// ## Constraints
    ///
    /// - The number of nodes in each linked list is in the range `[1, 100]`.
    /// - `0 <= Node.val <= 9`
    /// - It is guaranteed that the list represents a number that does not have leading zeros.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val;
                let (val, carry) = if sum < 10 {
                    (sum, None)
                } else {
                    (sum - 10, Some(Box::new(ListNode::new(1))))
                };
                Some(Box::new(ListNode {
                    val,
                    next: super::Solution::add_two_numbers(
                        super::Solution::add_two_numbers(l1.next, carry),
                        l2.next,
                    ),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list_node::ListNode;
    use crate::Solution;
    #[test]
    fn example_test_cases() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![2, 4, 3]))),
                Some(Box::new(ListNode::from(vec![5, 6, 4])))
            ),
            Some(Box::new(ListNode::from(vec![7, 0, 8])))
        );
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![0]))),
                Some(Box::new(ListNode::from(vec![0])))
            ),
            Some(Box::new(ListNode::from(vec![0])))
        );
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![9, 9, 9, 9, 9, 9, 9]))),
                Some(Box::new(ListNode::from(vec![9, 9, 9, 9])))
            ),
            Some(Box::new(ListNode::from(vec![8, 9, 9, 9, 0, 0, 0, 1])))
        );
    }
}
