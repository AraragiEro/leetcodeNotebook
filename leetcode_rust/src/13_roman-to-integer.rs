#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        for (index, c) in s.chars().enumerate() {
            let next_char = s.chars().nth(index + 1);

            match (c, next_char) {
                ('I', Some('V')) => result -= 1,
                ('I', Some('X')) => result -= 1,
                ('X', Some('L')) => result -= 10,
                ('X', Some('C')) => result -= 10,
                ('C', Some('D')) => result -= 100,
                ('C', Some('M')) => result -= 100,
                ('I', _) => result += 1,
                ('V', _) => result += 5,
                ('X', _) => result += 10,
                ('L', _) => result += 50,
                ('C', _) => result += 100,
                ('D', _) => result += 500,
                ('M', _) => result += 1000,
                _ => result += 0,
            }
        }

        return result;
    }
}

// @lc code=end

