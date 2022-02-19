#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(nums: Vec<i32>) -> Self {
        let mut head = ListNode::new(nums[0]);
        let mut cur = &mut head;
        for num in nums.iter().skip(1) {
            cur.next = Some(Box::new(ListNode::new(*num)));
            cur = cur.next.as_mut().unwrap();
        }
        head
    }
}
