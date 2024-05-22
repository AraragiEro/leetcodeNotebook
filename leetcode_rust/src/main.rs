fn main() {
    println!("Hello, world!");
}

#[path ="./1_two-sum.rs"]
mod two_sum;
#[path ="./13_roman-to-integer.rs"]
mod roman_to_integer;
//#[path ="./21_merge-two-sorted-lists.rs"]
//mod merge_two_sorted_lists;
#[path ="./66.plus-one.rs"]
mod plus_one;


#[cfg(test)]
mod diff_easy_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            two_sum::Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]
        );
    }

    #[test]
    fn test_13() {
        assert_eq!(
            roman_to_integer::Solution::roman_to_int("III".to_string()), 3
        );
    }

    #[test]
    fn test_27() {
        assert_eq!(
            remove_element::Solution::remove_element(&mut vec![1], 1), 0
        );
    }

}
