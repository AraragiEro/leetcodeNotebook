#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
// Definition for singly-linked list.
/* 
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
*/


impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(mut a), Some(mut b)) => {
                if a.val >= b.val {
                    a.next = Solution::merge_two_lists(a.next, Some(b));
                    return Some(a);
                } else {
                    b.next = Solution::merge_two_lists(Some(a), b.next);
                    return Some(b);
                }
            }
        }
    }
}
// @lc code=end

