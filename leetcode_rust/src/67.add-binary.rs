/*
 * @lc app=leetcode.cn id=67 lang=rust
 * @lcpr version=
 *
 * [67] 二进制求和
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let num_a = u128::from_str_radix(&a, 2);
        let num_b = u128::from_str_radix(&b, 2);
        
        match (num_a, num_b) {
            (Ok(a), Ok(b)) => {
                let res = a + b;
                format!("{res:b}")
            },
            (_, _) => {
                let mut res = String::new();
                let mut plus = 0;
                let mut a_chars: Vec<char> = a.chars().collect();
                let mut b_chars: Vec<char> = b.chars().collect();
                a_chars.reverse();
                b_chars.reverse();

                for i in 0..a_chars.len().max(b_chars.len()) {
                    let a_bit = a_chars.get(i).unwrap_or(&'0').to_digit(2).unwrap();
                    let b_bit = b_chars.get(i).unwrap_or(&'0').to_digit(2).unwrap();
                    let sum = a_bit + b_bit + plus;
                    res.push_str(&((sum % 2).to_string()));
                    plus = sum / 2;
                }
                
                if plus > 0 {
                    res.push_str(&plus.to_string());
                }
                res.chars().rev().collect()
            }
        }
    }
}
// @lc code=end



/*
// @lcpr case=start
// "11"\n"1"\n
// @lcpr case=end

// @lcpr case=start
// "1010"\n"1011"\n
// @lcpr case=end

 */

