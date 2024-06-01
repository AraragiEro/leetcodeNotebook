/*
 * @lc app=leetcode.cn id=69 lang=rust
 * @lcpr version=
 *
 * [69] x 的平方根 
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left: i128 = 0;
        let mut right: i128 = x as i128;

        while left < right - 1 {
            let mid = (left + right) / 2;
            let mid_sq = mid * mid;
            if mid_sq ==  x as i128 {
                return mid as i32;
            } else if mid_sq < x as i128 {
                left = mid;
            } else {
                right = mid
            }
        }

        if right * right > x as i128 {
            left as i32
        } else {
            right as i32
        }
    }
}
// @lc code=ends



/*
// @lcpr case=start
// 4\n
// @lcpr case=end

// @lcpr case=start
// 8\n
// @lcpr case=end

 */

