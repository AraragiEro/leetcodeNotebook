fn main() {
    println!("Hello, world!");
}

#[path ="./1_two-sum.rs"]
mod TwoSum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            TwoSum::Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]
        );
    }
}
