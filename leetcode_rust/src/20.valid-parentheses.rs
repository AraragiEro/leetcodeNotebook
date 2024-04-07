/*
 * @lc app=leetcode.cn id=20 lang=rust
 * @lcpr version=
 *
 * [20] 有效的括号
 */


// @lcpr-template-start

// @lcpr-template-end


// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let left_k = &['(', '[', '{'];
        
        let mut x = Vec::new();

        for i in s.chars() {
            if left_k.contains(&i) {
                x.push(i);
            } else {
                if let Some(last) = x.pop() {
                    match (last, i) {
                        ('(', ')') | ('[', ']') | ('{', '}') => continue,
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
        }
        if x.iter().count() != 0 {
            return false;
        }
        return true;
    }
}
// @lc code=end



/*
// @lcpr case=start
// "()"\n
// @lcpr case=end

// @lcpr case=start
// "()[]{}"\n
// @lcpr case=end

// @lcpr case=start
// "(]"\n
// @lcpr case=end

 */

