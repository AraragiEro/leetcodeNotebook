#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();
        for (i, v) in nums.iter().enumerate() {
            if i == length - 1 {
                break;
            }
            let child_vec = &nums[i+1..length];
            for (j, u) in child_vec.iter().enumerate() {
                if target == u + v {
                    return vec![i as i32, (i + j + 1) as i32];
                }
            }
        }
        return vec![]
    }
}
// @lc code=end

