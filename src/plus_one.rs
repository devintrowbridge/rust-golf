// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
// Increment the large integer by one and return the resulting array of digits.

// Example 1:
// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Incrementing by one gives 123 + 1 = 124.
// Thus, the result should be [1,2,4].


use std::collections::VecDeque;
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = true;
    let mut ans = VecDeque::from(digits);

    for n in ans.iter_mut().rev() {
        if carry {
            *n = (*n + 1) % 10;
            carry = *n == 0;
        }
    }
    if carry {
        ans.push_front(1);
    }

    ans.into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(plus_one(vec![1,2,3]),  vec![1,2,4]);
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(plus_one(vec![9]), vec![1,0]);
        assert_eq!(plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
    }
}