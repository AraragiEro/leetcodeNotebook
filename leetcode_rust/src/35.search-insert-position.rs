/*
 * @lc app=leetcode.cn id=35 lang=rust
 * @lcpr version=
 *
 * [35] 搜索插入位置
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
//pub struct Solution{}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,5,6]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,3,5,6]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,3,5,6]\n7\n
// @lcpr case=end

 */

