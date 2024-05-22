/*
 * @lc app=leetcode.cn id=58 lang=rust
 * @lcpr version=
 *
 * [58] 最后一个单词的长度
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
//pub struct Solution{}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last_word = s.split_whitespace().rev().next().unwrap();
        return last_word.len() as i32;
        
    }
}
// @lc code=end



/*
// @lcpr case=start
// "Hello World"\n
// @lcpr case=end

// @lcpr case=start
// "   fly me   to   the moon  "\n
// @lcpr case=end

// @lcpr case=start
// "luffy is still joyboy"\n
// @lcpr case=end

 */

