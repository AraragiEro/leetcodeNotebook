/*
 * @lc app=leetcode.cn id=66 lang=rust
 * @lcpr version=
 *
 * [66] 加一
 */


// @lcpr-template-start


// @lcpr-template-end
// @lc code=start
//struct Solution{}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut flag = true;
        let mut mut_digits: Vec<i32> = Vec::new();
        for number in digits.iter().rev() {
            println!("{}, {}", flag, number);
            match (flag, *number) {
                (true, 9) => {
                    mut_digits.push(0);
                    flag = true;
                },
                (true, _) => {
                    mut_digits.push(*number + 1);
                    flag = false;
                },
                (false, _) => {
                    mut_digits.push(*number);
                }
            }
        }
        
        if flag == true {
            mut_digits.push(1);
        }

        let output = mut_digits.into_iter().rev().collect();
        println!("{:?}", output);
        output
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [4,3,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [0]\n
// @lcpr case=end

 */

