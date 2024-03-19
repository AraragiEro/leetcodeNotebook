#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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

fn add_at_head(node_head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let new_node = ListNode {
        next: node_head,
        val: val
    };

    Some(Box::new(new_node))
}

pub fn test_list_1() -> Option<Box<ListNode>> {
    // [1579]
    let mut node_head = Some(Box::new(
        ListNode::new(1)
    ));

    node_head = add_at_head(node_head, 5);
    node_head = add_at_head(node_head, 7);
    node_head = add_at_head(node_head, 9);

    return node_head;
}

pub fn test_list_2() -> Option<Box<ListNode>> {
    // [12567]
    let mut node_head = Some(Box::new(
        ListNode::new(1)
    ));

    node_head = add_at_head(node_head, 2);
    node_head = add_at_head(node_head, 5);
    node_head = add_at_head(node_head, 6);
    node_head = add_at_head(node_head, 7);

    return node_head;
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &list1;
        let mut l2 = &list2;

        let result_list = None;

        while let l1 = Some(i) && l2 != Some(i) {
            result_list = add_at_head(result_list, l1.val)
        }

    }
}
// @lc code=end

