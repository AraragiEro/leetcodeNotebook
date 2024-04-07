#[allow(dead_code)]
/*
 * @lc app=leetcode.cn id=27 lang=rust
 * @lcpr version=
 *
 * [27] 移除元素
 */



// @lcpr-template-start
pub struct Solution {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut tail = (nums.len() - 1) as i32;

        for i in 0..nums.len() as i32 {
            if tail < i {
                break;
            }   
            while nums[tail as usize] == val && tail > i {
                tail -= 1;
            }
            if nums[i as usize] != val {
                continue;
            } else {
                nums[i as usize] = nums[tail as usize];
                tail -= 1;
            }
        }

        return (tail + 1) as i32;
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,2,2,3]\n3\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,2,3,0,4,2]\n2\n
// @lcpr case=end

 */

